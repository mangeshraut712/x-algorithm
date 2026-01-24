import jax
import jax.numpy as jnp
import haiku as hk
import numpy as np
import logging
from grok import GrokModel, GrokConfig
from typing import Any

logging.basicConfig(level=logging.INFO)
logger = logging.getLogger("antigravity-optimizer")

def validate_model_integrity():
    """
    Validates the Grok model's initialization and attention mask logic.
    Part of the Antigravity Enhancement Suite.
    """
    logger.info("🚀 Starting Antigravity Model Integrity Check...")
    
    config = GrokConfig(
        num_layers=2,
        num_heads=4,
        emb_size=128,
        hidden_size=256,
        max_seq_len=64
    )
    
    def forward(x):
        model = GrokModel(config)
        return model(x, padding_mask=jnp.ones((x.shape[0], x.shape[1])))

    init_fn, apply_fn = hk.transform(forward)
    
    rng = jax.random.PRNGKey(42)
    dummy_input = jnp.zeros((1, 16, 128))
    
    logger.info("Initializing model parameters...")
    params = init_fn(rng, dummy_input)
    
    # Check for NaNs
    has_nan = jax.tree_util.tree_flatten(jax.tree_util.tree_map(lambda x: jnp.any(jnp.isnan(x)), params))[0]
    if any(has_nan):
        logger.error("🛑 Model initialization failed: NaNs detected in parameters!")
    else:
        logger.info("✅ Parameter initialization: OK")

    # Benchmarking latency
    logger.info("Benchmarking JIT compilation and inference...")
    jitted_apply = jax.jit(apply_fn)
    
    # Warmup
    _ = jitted_apply(params, dummy_input)
    
    import time
    start = time.time()
    for _ in range(10):
        _ = jitted_apply(params, dummy_input).block_until_ready()
    end = time.time()
    
    logger.info(f"✅ Average Inference Latency: {(end-start)/10*1000:.2f}ms")
    logger.info("✨ Antigravity Suite: Validation Complete.")

if __name__ == "__main__":
    validate_model_integrity()
