//! # A Concordium V1 smart contract
use concordium_std::*;
use core::fmt::Debug;

/// Your smart contract state.
#[derive(Serialize, SchemaType, Clone)]
pub struct State {
    // Your state
    changed: bool,
}

/// Your smart contract errors.
#[derive(Debug, PartialEq, Eq, Reject, Serial, SchemaType)]
enum Error {
    /// Failed parsing the parameter.
    #[from(ParseError)]
    ParseParamsError,
    /// Your error
    AlreadyChanged,
}

/// Init function that creates a new smart contract.
#[init(contract = "hackaton_concordium_task_2_change_state")]
fn init<S: HasStateApi>(
    _ctx: &impl HasInitContext,
    _state_builder: &mut StateBuilder<S>,
) -> InitResult<State> {
    // Your code

    Ok(State {changed: false})
}

/// Receive function. The input parameter is the boolean variable `throw_error`.
///  If `throw_error == true`, the receive function will throw a custom error.
///  If `throw_error == false`, the receive function executes successfully.
#[receive(
    contract = "hackaton_concordium_task_2_change_state",
    name = "receive",
    parameter = "bool",
    error = "Error",
    mutable
)]
fn receive<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    _host: &mut impl HasHost<State, StateApiType = S>,
) -> Result<(), Error> {
    // Your code

    let do_change: bool = ctx.parameter_cursor().get()?; // Returns Error::ParseError on failure
    let state = _host.state_mut();

    if do_change == true && state.changed == true {
        Err(Error::AlreadyChanged)
    } else {
        state.changed = true;
        Ok(())
    }
}

/// View function that returns the content of the state.
#[receive(
    contract = "hackaton_concordium_task_2_change_state",
    name = "view",
    return_value = "State"
)]
fn view<'b, S: HasStateApi>(
    _ctx: &impl HasReceiveContext,
    host: &'b impl HasHost<State, StateApiType = S>,
) -> ReceiveResult<&'b State> {
    Ok(host.state())
}

#[concordium_cfg_test]
mod tests {
    use super::*;
    use test_infrastructure::*;

    type ContractResult<A> = Result<A, Error>;

    #[concordium_test]
    /// Test that initializing the contract succeeds with some state.
    fn test_init() {
        let ctx = TestInitContext::empty();

        let mut state_builder = TestStateBuilder::new();

        let state_result = init(&ctx, &mut state_builder);
        state_result.expect_report("Contract initialization results in error");
    }

    #[concordium_test]
    /// Test that invoking the `receive` endpoint with the `false` parameter
    /// succeeds in updating the contract.
    fn test_throw_no_error() {
        let ctx = TestInitContext::empty();

        let mut state_builder = TestStateBuilder::new();

        // Initializing state
        let initial_state = init(&ctx, &mut state_builder).expect("Initialization should pass");

        let mut ctx = TestReceiveContext::empty();

        let throw_error = false;
        let parameter_bytes = to_bytes(&throw_error);
        ctx.set_parameter(&parameter_bytes);

        let mut host = TestHost::new(initial_state, state_builder);

        // Call the contract function.
        let result: ContractResult<()> = receive(&ctx, &mut host);

        // Check the result.
        claim!(result.is_ok(), "Results in rejection");
    }

    #[concordium_test]
    /// Test that invoking the `receive` endpoint with the `true` parameter
    /// results in the `YourError` being thrown.
    fn test_throw_error() {
        let ctx = TestInitContext::empty();

        let mut state_builder = TestStateBuilder::new();

        // Initializing state
        let initial_state = init(&ctx, &mut state_builder).expect("Initialization should pass");

        let mut ctx = TestReceiveContext::empty();

        let changed = true;
        let parameter_bytes = to_bytes(&changed);
        ctx.set_parameter(&parameter_bytes);

        let mut host = TestHost::new(initial_state, state_builder);

        // Call the contract function.
        let _error: ContractResult<()> = receive(&ctx, &mut host);
        let error: ContractResult<()> = receive(&ctx, &mut host);

        // Check the result.
        claim_eq!(error, Err(Error::AlreadyChanged), "Function should throw an error.");
    }
}
