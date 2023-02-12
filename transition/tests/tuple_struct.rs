mod tests {
    use massa_serialization::{Serializer, Deserializer, SerializeError};
    use nom::{error::{ParseError, ContextError}, IResult};
    //TODO: make it  Optional import
    use transition::Versioned;
    #[test]
    fn tuple_struct() {

        #[transition::versioned(versions("1", "2"))]
        struct Wrap(u64);

        #[transition::impl_version(versions("1"))]
        impl Wrap {
            fn new() -> Self {
                Self(1)
            }
        }

        #[transition::impl_version(versions("2"))]
        impl Wrap {
            fn new() -> Self {
                Self(2)
            }
        }

        #[transition::impl_version(versions("1", "2"))]
        impl Wrap {
            fn get_a(&self) -> u64 {
                self.0
            }

            fn mul(&self, b: u64) -> u64 {
                self.0 * b
            }
        }

        let test = <Wrap!["2"]>::new();
        assert_eq!(test.get_a(), 2);
        assert_eq!(test.mul(2), 4);

        let test = <Wrap!["1"]>::new();
        assert_eq!(test.get_a(), 1);
        assert_eq!(test.mul(2), 2);

        struct WrapSerializer {}

        #[transition::impl_version(versions("1", "2"), structure = "Wrap")]
        impl Serializer<Wrap> for WrapSerializer {
            fn serialize(&self, data: &Wrap, buffer: &mut Vec<u8>) -> Result<(), SerializeError> {
                buffer.push(data.0 as u8);
                Ok(())
            }
        }

        struct WrapDeserializer {}

        #[transition::impl_version(versions("1", "2"), structure = "Wrap")]
        impl Deserializer<Wrap> for WrapDeserializer {
            fn deserialize<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
                    &self,
                    buffer: &'a [u8],
                ) -> IResult<&'a [u8], Wrap, E> {
                    // Not really implemented because lack of time
                    Ok((buffer, Wrap(0)))
            }
        }

    }
}