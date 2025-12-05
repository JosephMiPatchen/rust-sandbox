## look int light houses safe math
```
Safe Math in Consensus Crate
// ❌ Avoid - could panic
let result = a + b;

// ✅ Preferred
let result = a.saturating_add(b);
// or
use safe_arith::SafeArith;

let result = a.safe_add(b)?;
```

## look into how light house wants async and how async is handled in rust

Async Task Spawning for Blocking Work
// ❌ Avoid - blocking in async context
async fn some_handler() {
    let result = expensive_computation(); // blocks async runtime
}

// ✅ Preferred
async fn some_handler() {
    let result = tokio::task::spawn_blocking(|| {
        expensive_computation()
    }).await?;
}

## look into lighth houses span issues

// ❌ Avoid - span on simple getter
#[instrument]
fn get_head_block_root(&self) -> Hash256 {
    self.head_block_root
}

// ✅ Preferred - span on meaningful operations
#[instrument(skip(self))]
async fn process_block(&self, block: Block) -> Result<(), Error> {
    // meaningful computation
}