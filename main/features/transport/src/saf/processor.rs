//! SAF wrapper for the `Processor` trait.

/// Describe this processor unit — SAF wrapper for
/// [`crate::api::traits::Processor::describe`].
///
/// Returns the static label for the transport implementation.  Used for
/// logging, metrics, and health dashboards.
pub fn describe_processor(processor: &dyn crate::api::traits::Processor) -> &'static str {
    processor.describe()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestProcessor;
    impl crate::api::traits::Processor for TestProcessor {
        fn process(
            &self,
        ) -> futures::future::BoxFuture<'_, Result<(), crate::api::port::GrpcOutboundError>>
        {
            Box::pin(futures::future::ready(Ok(())))
        }
        fn describe(&self) -> &'static str {
            "test-processor"
        }
    }

    /// @covers: describe_processor
    #[test]
    fn test_describe_processor_delegates_to_impl() {
        let p = TestProcessor;
        assert_eq!(describe_processor(&p), "test-processor");
    }
}
