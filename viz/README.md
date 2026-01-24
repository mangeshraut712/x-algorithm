# X Algorithm Visualizer

An interactive educational tool designed to demystify the X Recommendation Algorithm. 

## Features
- **Interactive Pipeline Map**: Visualizes the flow of a post through sourcing, hydration, filtering, and scoring.
- **Attention Matrix Viz**: Demonstrates "Candidate Isolation"—a key design decision in the Grok-based transformer.
- **Engagement Simulator**: Live ranking simulation showing how different action weights (favorites vs replies) impact the final feed.

## How to use
1. Open `index.html` in any modern web browser.
2. Navigate through the tabs to explore different aspects of the system.
3. In the Simulator tab, adjust sliders to see real-time ranking updates.

## Design Philosophy
This tool follows the core principles found in the `x-algorithm` codebase:
- **No Hand-Engineered Features**: Emphasizing the move to transformer-based relevance.
- **Candidate Isolation**: Visualizing why scores are consistent and independent.
- **Multi-Action Prediction**: Showing how 15+ actions are combined into a single score.
