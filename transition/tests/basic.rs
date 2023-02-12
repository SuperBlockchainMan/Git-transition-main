mod tests {
    use massa_serialization::{Serializer, Deserializer, SerializeError};
    use nom::{error::{ParseError, ContextError}, IResult};
    //TODO: Make it optional
    use transition::Versioned;
    #[test]
    fn basic() {
        #[transition::versioned(versions("1", "2", "3"))]
        struct Test {
            a: u64,
            //TODO: Use add possibility to use <> in version number
            #[transition::field(versions("3"))]
            b: u64,
        }

        impl Test {
            fn new(version: u64) -> Self {
                match version {
                    <Test!["1"]>::VERSION => TestVariant!["1"](<Test!["1"]>::new()),
                    <Test!["2"]>::VERSION => TestVariant!["2"](<Test!["2"]>::new()),
                    <Test!["3"]>::VERSION => TestVariant!["3"](<Test!["3"]>::new()),
                    _ => panic!("Unknown version: {}", version)
                }
            }

            fn get_a(&self) -> u64 {
                match self {
                    Test::TestV1(test) => test.get_a(),
                    Test::TestV2(test) => test.get_a(),
                    Test::TestV3(test) => test.get_a(),
                }
            }
        }

        #[transition::impl_version(versions("1"))]
        impl Test {
            fn new() -> Self {
                Self { a: 1 }
            }
        }

        #[transition::impl_version(versions("2"))]
        impl Test {
            fn new() -> Self {
                Self { a: 2 }
            }
        }

        #[transition::impl_version(versions("3"))]
        impl Test {
            fn new() -> Self {
                Self { a: 2, b: 3 }
            }

            fn get_b(&self) -> u64 {
                self.b
            }
        }

        #[transition::impl_version(versions("1", "2", "3"))]
        impl Test {
            fn get_a(&self) -> u64 {
                Test::test_method(self.a)
            }

            fn mul(&self, b: u64) -> u64 {
                self.a * b
            }

            fn test_method(a: u64) -> u64 {
                a
            }
        }

        let test = <Test!["2"]>::new();
        assert_eq!(test.get_a(), 2);
        assert_eq!(test.mul(2), 4);

        let test = <Test!["1"]>::new();
        assert_eq!(test.get_a(), 1);
        assert_eq!(test.mul(2), 2);

        let test = <Test!["3"]>::new();
        assert_eq!(test.get_b(), 3);

        let test = Test::new(1);
        assert_eq!(test.get_a(), 1);

        struct TestSerializer {}

        #[transition::impl_version(versions("1", "2"), structures("Test"))]
        impl Serializer<Test> for TestSerializer {
            fn serialize(&self, data: &Test, buffer: &mut Vec<u8>) -> Result<(), SerializeError> {
                buffer.push(data.a as u8);
                Ok(())
            }
        }

        #[transition::impl_version(versions("3"), structures("Test"))]
        impl Serializer<Test> for TestSerializer {
            fn serialize(&self, data: &Test, buffer: &mut Vec<u8>) -> Result<(), SerializeError> {
                buffer.push(data.a as u8);
                buffer.push(data.b as u8);
                Ok(())
            }
        }

        struct TestDeserializer {}

        #[transition::impl_version(versions("1", "2"), structures("Test"))]
        impl Deserializer<Test> for TestDeserializer {
            fn deserialize<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
                    &self,
                    buffer: &'a [u8],
                ) -> IResult<&'a [u8], Test, E> {
                    // Not really implemented because lack of time
                    Ok((buffer, Test {a: 2}))
            }
        }

        #[transition::impl_version(versions("3"), structures("Test"))]
        impl Deserializer<Test> for TestDeserializer {
            fn deserialize<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
                    &self,
                    buffer: &'a [u8],
                ) -> IResult<&'a [u8], Test, E> {
                    // Compile error because `b` is not declared
                    //Ok((buffer, Test {a: 2}))
                    Ok((buffer, Test {a: 2, b: 3}))
            }
        }

    }
}