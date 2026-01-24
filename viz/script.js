document.addEventListener('DOMContentLoaded', () => {
    // State
    const state = {
        currentView: 'pipeline',
        activeStage: null,
        weights: {
            'Favorite': 1.0,
            'Reply': 2.5,
            'Repost': 1.5,
            'Quote': 2.0,
            'Click': 0.5,
            'Video View': 0.8,
            'Profile Click': 0.3,
            'Report': -20.0,
            'Not Interested': -10.0
        },
        mockPosts: [
            { id: 1, author: 'Elon Musk', handle: '@elonmusk', text: 'Humility is the path to greatness.', likes: '1.2M', replies: '45K', baseScore: 0.95, source: 'Thunder' },
            { id: 2, author: 'xAI', handle: '@xai', text: 'The new Grok-2 is now live with enhanced reasoning capabilities.', likes: '250K', replies: '12K', baseScore: 0.88, source: 'Phoenix Retrieval' },
            { id: 3, author: 'Tech Insider', handle: '@tech', text: 'Understanding the attention mechanism in recommendation systems.', likes: '15K', replies: '400', baseScore: 0.72, source: 'Phoenix Retrieval' },
            { id: 4, author: 'Daily Vibes', handle: '@vibes', text: 'The sunset today was absolutely breathtaking. 🌅', likes: '5K', replies: '50', baseScore: 0.45, source: 'Thunder' },
            { id: 5, author: 'Global News', handle: '@news', text: 'Breakthrough in quantum computing announced today.', likes: '85K', replies: '2K', baseScore: 0.81, source: 'Phoenix Retrieval' }
        ]
    };

    // DOM Elements
    const navButtons = document.querySelectorAll('.nav-btn');
    const views = document.querySelectorAll('.view');
    const stages = document.querySelectorAll('.stage');
    const detailsPanel = document.getElementById('stage-details');
    const matrixViz = document.getElementById('attention-matrix');
    const actionsViz = document.getElementById('actions-viz');
    const weightSliders = document.getElementById('weight-sliders');
    const mockFeed = document.getElementById('mock-feed');
    const resetWeightsBtn = document.getElementById('reset-weights');

    // Data Mapping for Pipeline Insights
    const stageInsights = {
        'stage-source': {
            title: 'Candidate Sourcing (Thunder & Phoenix)',
            desc: 'Candidates are fetched from two main sources. Thunder handles users you follow (in-network), while Phoenix uses ML similarity search for discovery (out-of-network).',
            code: `// From candidate-pipeline/source.rs
pub trait Source: Send + Sync {
    type Output;
    async fn fetch(&self, query: &Query) -> Result<Self::Output>;
}`
        },
        'stage-hydration': {
            title: 'Candidate Hydration',
            desc: 'Raw IDs are enriched with metadata. This includes author info, media entities, and engagement stats required for scoring.',
            code: `// From candidate-pipeline/hydrator.rs
pub trait Hydrator: Send + Sync {
    async fn hydrate(&self, candidates: &mut [Candidate]) -> Result<()>;
}`
        },
        'stage-filtering': {
            title: 'Visibility & Safety Filtering',
            desc: 'Multiple passes to ensure quality. Removes deleted posts, duplicates, and content from blocked or muted authors.',
            code: `// From candidate-pipeline/filter.rs
pub trait Filter: Send + Sync {
    fn filter(&self, candidates: &mut Vec<Candidate>);
}

// Example: MutedKeywordFilter, DropDuplicatesFilter`
        },
        'stage-scoring': {
            title: 'Scoring (Phoenix Transformer)',
            desc: 'The heart of the system. A Grok-based transformer predicts engagement probabilities (P_favorite, P_repost, etc.) for each post based on your history.',
            code: `# From phoenix/recsys_model.py
class PhoenixModel(hk.Module):
    def __call__(self, batch, embeddings):
        # Transformer predicts logits for multiple actions
        model_output = self.model(embeddings, padding_mask)
        logits = jnp.dot(candidate_embeddings, unembeddings)
        return logits`
        },
        'stage-selection': {
            title: 'Selection & Diversity',
            desc: 'Posts are sorted by the weighted score. Diversity adjustments ensure you dont see too many posts from the same author in a row.',
            code: `// From home-mixer/selectors/author_diversity.rs
pub struct AuthorDiversitySelector {
    pub min_author_gap: usize,
}`
        }
    };

    // Navigation logic
    navButtons.forEach(btn => {
        btn.addEventListener('click', () => {
            const viewId = btn.dataset.view;
            state.currentView = viewId;

            navButtons.forEach(b => b.classList.remove('active'));
            btn.classList.add('active');

            views.forEach(v => {
                v.classList.toggle('active', v.id === `${viewId}-view`);
            });

            if (viewId === 'model') renderModelView();
            if (viewId === 'simulator') renderSimulatorView();
        });
    });

    // Pipeline Logic
    stages.forEach(stage => {
        stage.addEventListener('click', () => {
            stages.forEach(s => s.classList.remove('active'));
            stage.classList.add('active');

            const insight = stageInsights[stage.id];
            if (insight) {
                renderDetails(insight);
            }
        });
    });

    function renderDetails(data) {
        detailsPanel.innerHTML = `
            <div class="detail-content">
                <h4>${data.title}</h4>
                <p>${data.desc}</p>
                <div class="code-snippet">${data.code.replace(/</g, '&lt;').replace(/>/g, '&gt;')}</div>
            </div>
        `;
    }

    // Model View logic
    function renderModelView() {
        // Render Attention Matrix
        matrixViz.innerHTML = '';
        const size = 12; // 12x12 grid
        const historySize = 4;
        const candidateStart = 4;

        for (let i = 0; i < size; i++) {
            for (let j = 0; j < size; j++) {
                const cell = document.createElement('div');
                cell.className = 'matrix-cell';

                // Masking logic based on make_recsys_attn_mask in grok.py
                if (i < candidateStart) {
                    // History can see past history entries (causal)
                    if (j <= i) cell.classList.add('history');
                    else cell.classList.add('mask');
                } else {
                    // Candidate can see all history
                    if (j < candidateStart) {
                        cell.classList.add('history');
                    }
                    // Candidate can ONLY see itself from the candidate pool
                    else if (j === i) {
                        cell.classList.add('cand');
                    }
                    else {
                        cell.classList.add('mask');
                    }
                }
                matrixViz.appendChild(cell);
            }
        }

        // Render Action Logic
        const actions = Object.entries(state.weights).filter(([_, w]) => w > 0);
        actionsViz.innerHTML = actions.map(([name, weight]) => `
            <div class="action-item">
                <span class="action-label">P(${name.toLowerCase()})</span>
                <div class="action-bar-container">
                    <div class="action-bar" style="width: ${Math.min(100, (weight / 3) * 100)}%"></div>
                </div>
                <span class="action-val">${(weight).toFixed(1)}</span>
            </div>
        `).join('');
    }

    // Simulator View logic
    function renderSimulatorView() {
        // Render Sliders
        weightSliders.innerHTML = Object.entries(state.weights).map(([name, weight]) => `
            <div class="slider-group">
                <div class="slider-header">
                    <label>${name}</label>
                    <span>${weight > 0 ? '+' : ''}${weight.toFixed(1)}</span>
                </div>
                <input type="range" min="${weight < 0 ? -30 : 0}" max="${weight < 0 ? 0 : 10}" step="0.1" value="${weight}" data-name="${name}">
            </div>
        `).join('');

        // Attach slider listeners
        const sliders = weightSliders.querySelectorAll('input');
        sliders.forEach(slider => {
            slider.addEventListener('input', (e) => {
                const name = e.target.dataset.name;
                state.weights[name] = parseFloat(e.target.value);

                // Update label
                const label = e.target.previousElementSibling.querySelector('span');
                label.textContent = `${state.weights[name] > 0 ? '+' : ''}${state.weights[name].toFixed(1)}`;

                updateFeed();
            });
        });

        updateFeed();
    }

    function updateFeed() {
        // Calculate scores
        const rankedPosts = state.mockPosts.map(post => {
            // Simplified scoring: baseScore * sum(random engagement likelihoods * weights)
            let score = post.baseScore * 10;

            // Artificial engagement prediction simulation
            score += (post.id % 3 === 0 ? 0.8 : 0.2) * state.weights['Favorite'];
            score += (post.id % 2 === 0 ? 0.3 : 0.1) * state.weights['Reply'];
            score += (Math.random() * 0.5) * state.weights['Repost'];

            // Penalize negative actions (probability is always low but impact is high)
            score += 0.01 * state.weights['Report'];

            return { ...post, finalScore: score };
        }).sort((a, b) => b.finalScore - a.finalScore);

        // Render posts
        mockFeed.innerHTML = rankedPosts.map((post, index) => `
            <div class="feed-item" style="animation-delay: ${index * 0.1}s">
                <div class="post-rank">#${index + 1}</div>
                <div class="post-content">
                    <div class="post-author">${post.author} <span>${post.handle} · ${post.source}</span></div>
                    <div class="post-text">${post.text}</div>
                    <div class="post-stats">
                        <span>💬 ${post.replies}</span>
                        <span>🔄 3.2K</span>
                        <span>❤️ ${post.likes}</span>
                    </div>
                </div>
                <div class="post-score">${post.finalScore.toFixed(2)}</div>
            </div>
        `).join('');
    }

    resetWeightsBtn.addEventListener('click', () => {
        state.weights = {
            'Favorite': 1.0,
            'Reply': 2.5,
            'Repost': 1.5,
            'Quote': 2.0,
            'Click': 0.5,
            'Video View': 0.8,
            'Profile Click': 0.3,
            'Report': -20.0,
            'Not Interested': -10.0
        };
        renderSimulatorView();
    });

    // Initial state
    renderModelView();
});
