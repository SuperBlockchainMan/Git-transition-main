#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
mod tests {
    use massa_serialization::{Serializer, Deserializer, SerializeError};
    use nom::{
        error::{ParseError, ContextError},
        IResult,
    };
    use transition::Versioned;
    extern crate test;
    #[cfg(test)]
    #[rustc_test_marker = "tests::basic"]
    pub const basic: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("tests::basic"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::IntegrationTest,
        },
        testfn: test::StaticTestFn(|| test::assert_test_result(basic())),
    };
    fn basic() {
        enum Test {
            TestV1(TestV1),
            TestV2(TestV2),
            TestV3(TestV3),
        }
        struct TestV1 {
            a: u64,
        }
        struct TestV2 {
            a: u64,
        }
        struct TestV3 {
            a: u64,
            b: u64,
        }
        impl Versioned for TestV1 {
            const VERSION: u64 = 1u64;
            const VERSION_VARINT_SIZE_BYTES: usize = 1usize;
        }
        impl Versioned for TestV2 {
            const VERSION: u64 = 2u64;
            const VERSION_VARINT_SIZE_BYTES: usize = 1usize;
        }
        impl Versioned for TestV3 {
            const VERSION: u64 = 3u64;
            const VERSION_VARINT_SIZE_BYTES: usize = 1usize;
        }
        impl Test {
            fn new(version: u64) -> Self {
                match version {
                    <TestV1>::VERSION => Test::TestV1(<TestV1>::new()),
                    <TestV2>::VERSION => Test::TestV2(<TestV2>::new()),
                    <TestV3>::VERSION => Test::TestV3(<TestV3>::new()),
                    _ => {
                        ::core::panicking::panic_fmt(
                            ::core::fmt::Arguments::new_v1(
                                &["Unknown version: "],
                                &[::core::fmt::ArgumentV1::new_display(&version)],
                            ),
                        )
                    }
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
        impl TestV1 {
            fn new() -> Self {
                Self { a: 1 }
            }
        }
        impl TestV2 {
            fn new() -> Self {
                Self { a: 2 }
            }
        }
        impl TestV3 {
            fn new() -> Self {
                Self { a: 2, b: 3 }
            }
            fn get_b(&self) -> u64 {
                self.b
            }
        }
        impl TestV1 {
            fn get_a(&self) -> u64 {
                TestV1::test_method(self.a)
            }
            fn mul(&self, b: u64) -> u64 {
                self.a * b
            }
            fn test_method(a: u64) -> u64 {
                a
            }
        }
        impl TestV2 {
            fn get_a(&self) -> u64 {
                TestV2::test_method(self.a)
            }
            fn mul(&self, b: u64) -> u64 {
                self.a * b
            }
            fn test_method(a: u64) -> u64 {
                a
            }
        }
        impl TestV3 {
            fn get_a(&self) -> u64 {
                TestV3::test_method(self.a)
            }
            fn mul(&self, b: u64) -> u64 {
                self.a * b
            }
            fn test_method(a: u64) -> u64 {
                a
            }
        }
        let test = <TestV2>::new();
        match (&test.get_a(), &2) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        match (&test.mul(2), &4) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        let test = <TestV1>::new();
        match (&test.get_a(), &1) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        match (&test.mul(2), &2) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        let test = <TestV3>::new();
        match (&test.get_b(), &3) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        let test = Test::new(1);
        match (&test.get_a(), &1) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        struct TestSerializer {}
        impl Serializer<TestV1> for TestSerializer {
            fn serialize(
                &self,
                data: &TestV1,
                buffer: &mut Vec<u8>,
            ) -> Result<(), SerializeError> {
                buffer.push(data.a as u8);
                Ok(())
            }
        }
        impl Serializer<TestV2> for TestSerializer {
            fn serialize(
                &self,
                data: &TestV2,
                buffer: &mut Vec<u8>,
            ) -> Result<(), SerializeError> {
                buffer.push(data.a as u8);
                Ok(())
            }
        }
        impl Serializer<TestV3> for TestSerializer {
            fn serialize(
                &self,
                data: &TestV3,
                buffer: &mut Vec<u8>,
            ) -> Result<(), SerializeError> {
                buffer.push(data.a as u8);
                buffer.push(data.b as u8);
                Ok(())
            }
        }
        struct TestDeserializer {}
        impl Deserializer<TestV1> for TestDeserializer {
            fn deserialize<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
                &self,
                buffer: &'a [u8],
            ) -> IResult<&'a [u8], TestV1, E> {
                Ok((buffer, TestV1 { a: 2 }))
            }
        }
        impl Deserializer<TestV2> for TestDeserializer {
            fn deserialize<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
                &self,
                buffer: &'a [u8],
            ) -> IResult<&'a [u8], TestV2, E> {
                Ok((buffer, TestV2 { a: 2 }))
            }
        }
        impl Deserializer<TestV3> for TestDeserializer {
            fn deserialize<'a, E: ParseError<&'a [u8]> + ContextError<&'a [u8]>>(
                &self,
                buffer: &'a [u8],
            ) -> IResult<&'a [u8], TestV3, E> {
                Ok((buffer, TestV3 { a: 2, b: 3 }))
            }
        }
    }
}
#[rustc_main]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(&[&basic])
}
