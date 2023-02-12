mod tests {
    use massa_serialization::{Serializer, Deserializer, SerializeError};
    use nom::{error::{ParseError, ContextError}, IResult};
    //TODO: Make it optional
    use transition::Versioned;
    #[test]
    fn enum_test() {

        #[transition::versioned_enum(versions("1", "2", "3"))]
        enum Human {
            Alice(Alice),
            Bob(Bob),
            #[transition::variant(versions("3"))]
            Charlie(Charlie),
        }

        struct Alice {
            a: u64,
        }

        struct Bob {
            b: u64,
        }

        struct Charlie {
            c: u64,
        }

        #[transition::impl_version(versions("1"))]
        impl Human {
            fn new() -> Self {
                Self::Alice(Alice { a: 1 })
            }
        }

        #[transition::impl_version(versions("2"))]
        impl Human {
            fn new() -> Self {
                Self::Bob(Bob { b: 2 })
            }
        }

        #[transition::impl_version(versions("3"))]
        impl Human {
            fn new() -> Self {
                Self::Charlie(Charlie { c: 3 })
            }
        }        

    }
}