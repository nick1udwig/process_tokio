use uqbar_process_lib::timer::set_and_await_timer;
use uqbar_process_lib::uqbar::process::standard as wit;

wit_bindgen::generate!({
    path: "wit",
    world: "process",
    exports: {
        world: Component,
    },
});

async fn local() {
    let local = tokio::task::LocalSet::new();
    //local.spawn_local(foo());
    //local.spawn_local(bar());
    local.spawn_local(foo2());
    local.spawn_local(bar2());

    local.await;
}

//async fn foo() {
//    loop {
//        let now = std::time::SystemTime::now();
//        wit::print_to_terminal(0, "process_tokio|foo: 0");
//        tokio::task::yield_now().await;
//        wit::print_to_terminal(0, &format!("process_tokio|foo: 1 {} since 0", now.elapsed().unwrap().as_millis()));
//        set_and_await_timer(1000).unwrap().unwrap();
//        wit::print_to_terminal(0, &format!("process_tokio|foo: 2 {} since 0", now.elapsed().unwrap().as_millis()));
//        tokio::task::yield_now().await;
//        wit::print_to_terminal(0, &format!("process_tokio|foo: 3 {} since 0", now.elapsed().unwrap().as_millis()));
//        tokio::task::yield_now().await;
//    }
//}
//
//async fn bar() {
//    loop {
//        let now = std::time::SystemTime::now();
//        wit::print_to_terminal(0, "process_tokio|bar: 0");
//        tokio::task::yield_now().await;
//        wit::print_to_terminal(0, &format!("process_tokio|bar: 1 {} since 0", now.elapsed().unwrap().as_millis()));
//        set_and_await_timer(1100).unwrap().unwrap();
//        wit::print_to_terminal(0, &format!("process_tokio|bar: 2 {} since 0", now.elapsed().unwrap().as_millis()));
//        tokio::task::yield_now().await;
//        wit::print_to_terminal(0, &format!("process_tokio|bar: 3 {} since 0", now.elapsed().unwrap().as_millis()));
//        tokio::task::yield_now().await;
//    }
//}

async fn foo2() {
    tokio::task::spawn_local(async { set_and_await_timer(500).unwrap().unwrap() }).await.unwrap();
    loop {
        let now = std::time::SystemTime::now();
        wit::print_to_terminal(0, &format!("process_tokio|foo2: start"));
        tokio::task::spawn_local(async move {
            let desired_sleep_time = std::time::Duration::from_millis(1000);
            let actual_sleep_time = desired_sleep_time.saturating_sub(now.elapsed().unwrap()).as_millis();
            wit::print_to_terminal(0, &format!("process_tokio|foo2: sleeping {} actual to hit {} desired", actual_sleep_time, desired_sleep_time.as_millis()));
            if actual_sleep_time > 0 {
                set_and_await_timer(actual_sleep_time as u64).unwrap().unwrap();
            }
            wit::print_to_terminal(0, &format!("process_tokio|foo2: {} elapsed", now.elapsed().unwrap().as_millis()));
        }).await.unwrap();
    }
}

async fn bar2() {
    loop {
        let now = std::time::SystemTime::now();
        wit::print_to_terminal(0, &format!("process_tokio|bar2: start"));
        tokio::task::spawn_local(async move {
            let desired_sleep_time = std::time::Duration::from_millis(1500);
            let actual_sleep_time = desired_sleep_time.saturating_sub(now.elapsed().unwrap()).as_millis();
            wit::print_to_terminal(0, &format!("process_tokio|bar: sleeping {} actual to hit {} desired", actual_sleep_time, desired_sleep_time.as_millis()));
            if actual_sleep_time > 0 {
                set_and_await_timer(actual_sleep_time as u64).unwrap().unwrap();
            }
            wit::print_to_terminal(0, &format!("process_tokio|bar2: {} elapsed", now.elapsed().unwrap().as_millis()));
        }).await.unwrap();
    }
}

struct Component;
impl Guest for Component {
    fn init(_our: String) {
        wit::print_to_terminal(0, "process_tokio: begin");

        let rt = tokio::runtime::Builder::new_current_thread()
            .build()
            .unwrap();

        rt.block_on(local());
    }
}
