# 🚀 X Algorithm Posting Strategy Guide
## Based on the Actual Open-Source Algorithm Code

*Last Updated: January 2026*

---

## 📊 How Your Score is Actually Calculated

The algorithm uses a **Grok-based transformer** to predict engagement probabilities. Your final score is:

```
Score = Σ (weight × P(action))
```

### The Actual Weights from the Code (`params.rs`)

| Action | Weight | Strategic Importance |
|--------|--------|---------------------|
| **Reply** | **27.0** | 🔥🔥🔥 HIGHEST |
| **Profile Click** | **12.0** | 🔥🔥 Very High |
| **Favorite (Like)** | 1.0 | Standard |
| **Retweet** | 1.0 | Standard |
| **Quote Tweet** | 0.0* | Tracked separately |
| **VQV (Video Quality View)** | 0.3 | Bonus for videos |
| **Not Interested** | **-74.0** | ⚠️ VERY NEGATIVE |
| **Report** | **-369.0** | ☠️ ACCOUNT KILLER |

*These are the actual weights from the algorithm code*

---

## 🎯 The Action Hierarchy (What Actually Matters)

### Tier 1: High-Impact Actions (Focus Here)
1. **Replies + Author Response** → 27x more valuable than likes
2. **Profile Clicks → Follows** → 12x weight, signals genuine interest
3. **Bookmarks** → Strong intent signal (save for later)
4. **Shares via DM** → Indicates "send this to someone" worthy content

### Tier 2: Standard Signals
5. **Reposts/Retweets** → Distribution amplifiers
6. **Quote Tweets** → Adds conversation value
7. **Likes** → Good, but just the baseline

### Tier 3: Dwell Signals
8. **Video Watch Time** → VQV bonus after 2+ second threshold
9. **Image Expansion** → Photo expand is a tracked signal
10. **Thread Reading** → Only counts if they actually scroll

### ☠️ Tier 4: Negative Signals (AVOID AT ALL COSTS)
- **"Not Interested" clicks** → -74.0 weight
- **Blocks** → Severe penalty
- **Mutes** → Severe penalty  
- **Reports** → -369.0 weight (near-instant death)

---

## 🧠 The SimClusters System (Stay In Your Lane)

The algorithm uses **SimClusters** to categorize users and content. Here's what this means:

```
IF your content matches user's SimCluster → BOOST
IF your content is outside user's interests → ZERO distribution
```

### Strategic Implications:
- ✅ **Pick ONE niche and dominate it** (crypto, tech, AI, fitness, etc.)
- ❌ **Don't topic-switch** - posting about crypto then cooking confuses the algorithm
- ✅ **Your engaged audience trains the algorithm** - quality followers matter
- ❌ **Random viral posts outside your niche** can actually hurt you long-term

---

## 📱 Content Strategy Framework

### The DWELL Formula
```
DWELL = Time spent on your post = Algorithm gold
```

**High-Dwell Content Types:**
1. **Threads** (but ONLY if people read them)
2. **Videos over 2 seconds** (VQV bonus kicks in)
3. **Carousels/Multi-image posts** (expansion = signal)
4. **Controversial hooks** (stop the scroll)
5. **Questions that demand answers**

### Content Templates That Win

#### 1. The Controversy Hook
```
[Controversial statement about your niche]

Here's why everyone is wrong:

🧵
```
*Forces replies and engagement*

#### 2. The Value Thread
```
I spent [X hours/years] learning [topic].

Here's everything I know in 2 minutes:

🧵 (save this)
```
*Triggers bookmarks and profile clicks*

#### 3. The Question Post
```
Genuine question for [niche] people:

[Specific question that requires expertise to answer]

I'll reply to every comment.
```
*Generates reply chains with your responses*

#### 4. The Share Trigger
```
Send this to someone who needs to hear this:

[Insight/advice that's DM-worthy]
```
*Triggers share via DM signal*

---

## ⏰ Posting Cadence (The AuthorDiversityScorer)

The algorithm has an **AuthorDiversityScorer** that penalizes spam:

```
Each additional post from you in a feed = decaying multiplier
```

### Optimal Strategy:
- ✅ **3-5 high-quality posts per day** (spaced 3-4 hours apart)
- ❌ **NOT 20 low-engagement posts** (quality > quantity)
- ✅ **Fresh content wins** (AgeFilter removes old posts)
- ✅ **Space your posts** to avoid multiplier decay

---

## 🌐 In-Network vs Out-of-Network

The algorithm treats these **completely differently**:

| Type | Source | Weight Factor |
|------|--------|---------------|
| **In-Network** | Thunder (people you follow) | Full weight |
| **Out-of-Network** | Phoenix Retrieval (discovery) | OON_WEIGHT_FACTOR (discounted) |

### Growth Strategy:
1. **First**, maximize in-network engagement (your followers)
2. **Then**, the algorithm pushes you out-of-network
3. Strong in-network = better out-of-network distribution

---

## 🔗 The Link Penalty (Real Talk)

```
Links = Visibility killer
```

### Why?
- The algorithm wants to keep users ON the platform
- External links = potential exit = lower score

### Workarounds:
- ✅ Put links in your **bio** or **pinned post**
- ✅ Say "link in bio" or "link in replies"
- ✅ Use link in the **first reply** to yourself
- ❌ Never put links in the main post body

---

## 📋 Daily Execution Checklist

### Morning Post (8-10 AM local)
- [ ] Value-add thread or insight
- [ ] No links in body
- [ ] Strong hook in first line
- [ ] CTA for replies/saves

### Midday Engagement (12-2 PM)
- [ ] Reply to ALL comments from morning post
- [ ] Engage with similar accounts in your niche
- [ ] Quote tweet 1-2 relevant posts with your take

### Evening Post (6-8 PM)
- [ ] Controversial take or question
- [ ] Designed for maximum replies
- [ ] Commit to replying to everyone

### Before Bed
- [ ] Review what worked
- [ ] Plan tomorrow's content
- [ ] Final engagement sweep

---

## 🎯 TL;DR Priority Order

Based on the actual algorithm weights:

1. **Create reply/conversation-worthy content** (27x weight)
2. **Make it shareable** (especially DM-worthy)
3. **Maximize time spent viewing** (dwell time is tracked)
4. **Avoid triggering blocks/mutes** (-74 to -369 weight)
5. **Post fresh, space out your content** (AgeFilter + DiversityScorer)
6. **Stay in your niche** (SimClusters are real)
7. **Reply to your own comments** (author response = huge boost)

---

## 📈 Metrics to Track

| Metric | What It Tells You |
|--------|-------------------|
| **Reply Rate** | Are you sparking conversation? |
| **Profile Visits** | Are people curious about you? |
| **Bookmark Rate** | Is this save-worthy? |
| **Share Rate** | Would someone DM this? |
| **Block/Mute Rate** | Are you being annoying? |

---

## 🚫 What NOT To Do

1. ❌ **Engagement bait** that triggers "Not Interested"
2. ❌ **Posting links in the main post**
3. ❌ **Ignoring your comments** (reply chains = gold)
4. ❌ **Topic switching** (stay in your lane)
5. ❌ **Posting 20x/day** (quality > quantity)
6. ❌ **Controversial for controversy's sake** (blocks kill you)
7. ❌ **Buying fake engagement** (low-quality = low-quality reach)

---

*This guide is based on the actual X Algorithm codebase. The weights and systems described are derived from the open-source implementation.*

**Star the repo:** [github.com/mangeshraut712/x-algorithm](https://github.com/mangeshraut712/x-algorithm)
