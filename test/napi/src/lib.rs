use neon::prelude::*;

use crate::js::{
    arrays::*, boxed::*, coercions::*, date::*, errors::*, functions::*, numbers::*, objects::*,
    strings::*, threads::*, types::*,
};

mod js {
    pub mod arrays;
    pub mod boxed;
    pub mod coercions;
    pub mod date;
    pub mod errors;
    pub mod functions;
    pub mod numbers;
    pub mod objects;
    pub mod strings;
    pub mod threads;
    pub mod types;
    pub mod workers;
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    let greeting = cx.string("Hello, World!");
    let greeting_copy = greeting.value(&mut cx);
    let greeting_copy = cx.string(greeting_copy);

    cx.export_value("greeting", greeting)?;
    cx.export_value("greetingCopy", greeting_copy)?;

    // Global singletons.
    let undefined = cx.undefined();
    let null = cx.null();
    let b_true = cx.boolean(true);
    let b_false = cx.boolean(false);

    assert_eq!(b_true.value(&mut cx), true);
    assert_eq!(b_false.value(&mut cx), false);

    cx.export_value("undefined", undefined)?;
    cx.export_value("null", null)?;
    cx.export_value("true", b_true)?;
    cx.export_value("false", b_false)?;

    let one = cx.number(1);
    let two = cx.number(2.1);
    assert!((one.value(&mut cx) - 1.0).abs() < f64::EPSILON);
    assert!((two.value(&mut cx) - 2.1).abs() < f64::EPSILON);
    cx.export_value("one", one)?;
    cx.export_value("two", two)?;

    // Plain objects.
    let rust_created = cx.empty_object();
    {
        let a = cx.number(1);
        // set at name
        rust_created.set(&mut cx, "a", a)?;
        // set at index
        rust_created.set(&mut cx, 0, a)?;
    }
    {
        let whatever = cx.boolean(true);
        rust_created.set(&mut cx, "whatever", whatever)?;
    }

    assert!(
        ({
            let v: Handle<JsNumber> = rust_created.get(&mut cx, "a")?;
            v.value(&mut cx)
        } - 1.0f64)
            .abs()
            < f64::EPSILON
    );
    assert!(
        ({
            let v: Handle<JsNumber> = rust_created.get(&mut cx, 0)?;
            v.value(&mut cx)
        } - 1.0f64)
            .abs()
            < f64::EPSILON
    );
    assert_eq!(
        {
            let v: Handle<JsBoolean> = rust_created.get(&mut cx, "whatever")?;
            v.value(&mut cx)
        },
        true
    );

    let property_names = rust_created
        .get_own_property_names(&mut cx)?
        .to_vec(&mut cx)?
        .into_iter()
        .map(|value| {
            let string: Handle<JsString> = value.downcast_or_throw(&mut cx)?;
            Ok(string.value(&mut cx))
        })
        .collect::<Result<Vec<_>, _>>()?;
    assert_eq!(property_names, &["0", "a", "whatever"]);

    cx.export_value("rustCreated", rust_created)?;

    fn add1(mut cx: FunctionContext) -> JsResult<JsNumber> {
        let x = cx.argument::<JsNumber>(0)?.value(&mut cx);
        Ok(cx.number(x + 1.0))
    }

    cx.export_function("add1", add1)?;

    cx.export_function("return_js_string", return_js_string)?;
    cx.export_function("run_string_as_script", run_string_as_script)?;

    cx.export_function("return_js_number", return_js_number)?;
    cx.export_function("return_large_js_number", return_large_js_number)?;
    cx.export_function("return_negative_js_number", return_negative_js_number)?;
    cx.export_function("return_float_js_number", return_float_js_number)?;
    cx.export_function(
        "return_negative_float_js_number",
        return_negative_float_js_number,
    )?;
    cx.export_function("accept_and_return_js_number", accept_and_return_js_number)?;
    cx.export_function(
        "accept_and_return_large_js_number",
        accept_and_return_large_js_number,
    )?;
    cx.export_function(
        "accept_and_return_float_js_number",
        accept_and_return_float_js_number,
    )?;
    cx.export_function(
        "accept_and_return_negative_js_number",
        accept_and_return_negative_js_number,
    )?;

    cx.export_function("return_js_function", return_js_function)?;
    cx.export_function("call_js_function", call_js_function)?;
    cx.export_function(
        "call_js_function_idiomatically",
        call_js_function_idiomatically,
    )?;
    cx.export_function(
        "call_js_function_with_zero_args",
        call_js_function_with_zero_args,
    )?;
    cx.export_function(
        "call_js_function_with_one_arg",
        call_js_function_with_one_arg,
    )?;
    cx.export_function(
        "call_js_function_with_two_args",
        call_js_function_with_two_args,
    )?;
    cx.export_function(
        "call_js_function_with_three_args",
        call_js_function_with_three_args,
    )?;
    cx.export_function(
        "call_js_function_with_four_args",
        call_js_function_with_four_args,
    )?;
    cx.export_function(
        "call_js_function_with_custom_this",
        call_js_function_with_custom_this,
    )?;
    cx.export_function(
        "call_js_function_with_implicit_this",
        call_js_function_with_implicit_this,
    )?;
    cx.export_function(
        "exec_js_function_with_implicit_this",
        exec_js_function_with_implicit_this,
    )?;
    cx.export_function(
        "call_js_function_with_heterogeneous_tuple",
        call_js_function_with_heterogeneous_tuple,
    )?;
    cx.export_function("construct_js_function", construct_js_function)?;
    cx.export_function(
        "construct_js_function_idiomatically",
        construct_js_function_idiomatically,
    )?;
    cx.export_function(
        "construct_js_function_with_overloaded_result",
        construct_js_function_with_overloaded_result,
    )?;
    cx.export_function("num_arguments", num_arguments)?;
    cx.export_function("return_this", return_this)?;
    cx.export_function("require_object_this", require_object_this)?;
    cx.export_function("is_argument_zero_some", is_argument_zero_some)?;
    cx.export_function("require_argument_zero_string", require_argument_zero_string)?;
    cx.export_function("check_string_and_number", check_string_and_number)?;
    cx.export_function("execute_scoped", execute_scoped)?;
    cx.export_function("compute_scoped", compute_scoped)?;

    cx.export_function("return_js_array", return_js_array)?;
    cx.export_function("return_js_array_with_number", return_js_array_with_number)?;
    cx.export_function("return_js_array_with_string", return_js_array_with_string)?;
    cx.export_function("read_js_array", read_js_array)?;

    cx.export_function("to_string", to_string)?;

    cx.export_function("return_js_global_object", return_js_global_object)?;
    cx.export_function("return_js_object", return_js_object)?;
    cx.export_function("return_js_object_with_number", return_js_object_with_number)?;
    cx.export_function("return_js_object_with_string", return_js_object_with_string)?;
    cx.export_function(
        "return_js_object_with_mixed_content",
        return_js_object_with_mixed_content,
    )?;

    cx.export_function("return_array_buffer", return_array_buffer)?;
    cx.export_function("read_array_buffer_with_lock", read_array_buffer_with_lock)?;
    cx.export_function(
        "read_array_buffer_with_borrow",
        read_array_buffer_with_borrow,
    )?;
    cx.export_function("write_array_buffer_with_lock", write_array_buffer_with_lock)?;
    cx.export_function(
        "write_array_buffer_with_borrow_mut",
        write_array_buffer_with_borrow_mut,
    )?;
    cx.export_function("read_typed_array_with_borrow", read_typed_array_with_borrow)?;
    cx.export_function(
        "write_typed_array_with_borrow_mut",
        write_typed_array_with_borrow_mut,
    )?;
    cx.export_function("read_u8_typed_array", read_u8_typed_array)?;
    cx.export_function("copy_typed_array", copy_typed_array)?;
    cx.export_function("return_uninitialized_buffer", return_uninitialized_buffer)?;
    cx.export_function("return_buffer", return_buffer)?;
    cx.export_function("return_external_buffer", return_external_buffer)?;
    cx.export_function("return_external_array_buffer", return_external_array_buffer)?;
    cx.export_function("read_buffer_with_lock", read_buffer_with_lock)?;
    cx.export_function("read_buffer_with_borrow", read_buffer_with_borrow)?;
    cx.export_function("write_buffer_with_lock", write_buffer_with_lock)?;
    cx.export_function("write_buffer_with_borrow_mut", write_buffer_with_borrow_mut)?;
    cx.export_function("byte_length", byte_length)?;
    cx.export_function("call_nullary_method", call_nullary_method)?;
    cx.export_function("call_unary_method", call_unary_method)?;
    cx.export_function("call_symbol_method", call_symbol_method)?;

    cx.export_function("create_date", create_date)?;
    cx.export_function("get_date_value", get_date_value)?;
    cx.export_function("check_date_is_invalid", check_date_is_invalid)?;
    cx.export_function("check_date_is_valid", check_date_is_valid)?;
    cx.export_function("try_new_date", try_new_date)?;
    cx.export_function("try_new_lossy_date", try_new_lossy_date)?;
    cx.export_function("nan_dates", nan_dates)?;
    cx.export_function("create_date_from_value", create_date_from_value)?;
    cx.export_function("create_and_get_invalid_date", create_and_get_invalid_date)?;

    cx.export_function("is_array", is_array)?;
    cx.export_function("is_array_buffer", is_array_buffer)?;
    cx.export_function("is_uint32_array", is_uint32_array)?;
    cx.export_function("is_boolean", is_boolean)?;
    cx.export_function("is_buffer", is_buffer)?;
    cx.export_function("is_error", is_error)?;
    cx.export_function("is_null", is_null)?;
    cx.export_function("is_number", is_number)?;
    cx.export_function("is_object", is_object)?;
    cx.export_function("is_string", is_string)?;
    cx.export_function("is_undefined", is_undefined)?;
    cx.export_function("strict_equals", strict_equals)?;

    cx.export_function("new_error", new_error)?;
    cx.export_function("new_type_error", new_type_error)?;
    cx.export_function("new_range_error", new_range_error)?;
    cx.export_function("throw_error", throw_error)?;
    cx.export_function("downcast_error", downcast_error)?;

    cx.export_function("panic", panic)?;
    cx.export_function("panic_after_throw", panic_after_throw)?;

    cx.export_function("throw_and_catch", throw_and_catch)?;
    cx.export_function("call_and_catch", call_and_catch)?;
    cx.export_function("get_number_or_default", get_number_or_default)?;
    cx.export_function("is_construct", is_construct)?;
    cx.export_function("caller_with_drop_callback", caller_with_drop_callback)?;

    cx.export_function("count_called", {
        let n = std::cell::RefCell::new(0);

        move |mut cx| {
            *n.borrow_mut() += 1;

            Ok(cx.number(*n.borrow()))
        }
    })?;

    fn call_get_own_property_names(mut cx: FunctionContext) -> JsResult<JsArray> {
        let object = cx.argument::<JsObject>(0)?;
        object.get_own_property_names(&mut cx)
    }

    cx.export_function("get_own_property_names", call_get_own_property_names)?;

    cx.export_function("person_new", person_new)?;
    cx.export_function("person_greet", person_greet)?;
    cx.export_function("ref_person_new", ref_person_new)?;
    cx.export_function("ref_person_greet", ref_person_greet)?;
    cx.export_function("ref_person_set_name", ref_person_set_name)?;
    cx.export_function("ref_person_fail", ref_person_fail)?;
    cx.export_function("external_unit", external_unit)?;

    cx.export_function("useless_root", useless_root)?;
    cx.export_function("thread_callback", thread_callback)?;
    cx.export_function("multi_threaded_callback", multi_threaded_callback)?;
    cx.export_function("greeter_new", greeter_new)?;
    cx.export_function("greeter_greet", greeter_greet)?;
    cx.export_function("leak_channel", leak_channel)?;
    cx.export_function("drop_global_queue", drop_global_queue)?;
    cx.export_function("channel_join", channel_join)?;
    cx.export_function("sum", sum)?;
    cx.export_function("sum_manual_promise", sum_manual_promise)?;
    cx.export_function("sum_rust_thread", sum_rust_thread)?;
    cx.export_function("leak_promise", leak_promise)?;
    cx.export_function("channel_panic", channel_panic)?;
    cx.export_function("channel_throw", channel_throw)?;
    cx.export_function("channel_panic_throw", channel_panic_throw)?;
    cx.export_function("channel_custom_panic", channel_custom_panic)?;
    cx.export_function("custom_panic_downcast", custom_panic_downcast)?;
    cx.export_function("task_panic_execute", task_panic_execute)?;
    cx.export_function("task_panic_complete", task_panic_complete)?;
    cx.export_function("task_throw", task_throw)?;
    cx.export_function("task_panic_throw", task_panic_throw)?;
    cx.export_function("task_custom_panic", task_custom_panic)?;
    cx.export_function("task_reject_promise", task_reject_promise)?;
    cx.export_function("task_panic_execute_promise", task_panic_execute_promise)?;
    cx.export_function("task_panic_complete_promise", task_panic_complete_promise)?;
    cx.export_function("task_panic_throw_promise", task_panic_throw_promise)?;
    cx.export_function("deferred_settle_with_throw", deferred_settle_with_throw)?;
    cx.export_function("deferred_settle_with_panic", deferred_settle_with_panic)?;
    cx.export_function(
        "deferred_settle_with_panic_throw",
        deferred_settle_with_panic_throw,
    )?;
    cx.export_function("get_and_replace", js::workers::get_and_replace)?;
    cx.export_function("get_or_init", js::workers::get_or_init)?;
    cx.export_function("get_or_init_clone", js::workers::get_or_init_clone)?;

    Ok(())
}
