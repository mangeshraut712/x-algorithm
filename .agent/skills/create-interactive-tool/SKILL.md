---
name: create-interactive-tool
description: Create a standalone interactive HTML tool with modern styling
---

# Create Interactive Tool Skill

Create beautiful, standalone HTML tools with no dependencies.

## When to Use

Use when creating:
- Calculators
- Visualizations
- Analyzers
- Interactive documentation

## Design Principles

1. **No Dependencies** - Pure HTML/CSS/JS
2. **Dark Theme** - Modern dark UI
3. **Responsive** - Works on mobile
4. **Interactive** - Real-time updates
5. **Cross-Browser** - Standard CSS properties

## Template Structure

```html
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Tool Name</title>
    <style>
        * { margin: 0; padding: 0; box-sizing: border-box; }
        
        body {
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
            background: linear-gradient(135deg, #0f0f0f 0%, #1a1a2e 100%);
            min-height: 100vh;
            color: #e0e0e0;
            padding: 20px;
        }
        
        .container { max-width: 900px; margin: 0 auto; }
        
        h1 {
            background: linear-gradient(135deg, #1da1f2, #00d4aa);
            -webkit-background-clip: text;
            background-clip: text;
            -webkit-text-fill-color: transparent;
            color: transparent;
        }
        
        .card {
            background: rgba(255, 255, 255, 0.03);
            border: 1px solid rgba(255, 255, 255, 0.1);
            border-radius: 16px;
            padding: 24px;
            margin-bottom: 20px;
        }
    </style>
</head>
<body>
    <div class="container">
        <h1>Tool Title</h1>
        <!-- Content here -->
    </div>
    <script>
        // Interactive logic
    </script>
</body>
</html>
```

## Color Palette

| Color | Hex | Use |
|-------|-----|-----|
| Primary | #1da1f2 | Buttons, accents |
| Success | #00d4aa | Positive values |
| Warning | #feca57 | Caution |
| Danger | #f44336 | Errors, negative |
| Background | #0f0f0f | Body |
| Card | rgba(255,255,255,0.03) | Containers |
| Text | #e0e0e0 | Body text |
| Muted | #888 | Secondary text |

## CSS Compatibility

Always include both prefixed and standard properties:

```css
/* Gradient text */
background: linear-gradient(...);
-webkit-background-clip: text;
background-clip: text;
-webkit-text-fill-color: transparent;
color: transparent;

/* Custom appearances */
-webkit-appearance: none;
appearance: none;
```

## Interactive Elements

### Range Sliders
```html
<input type="range" min="0" max="100" value="50" oninput="calculate()">
```

### Real-time Updates
```javascript
function calculate() {
    const value = document.getElementById('input').value;
    document.getElementById('output').textContent = processValue(value);
}
```

## File Naming

- Use lowercase with hyphens: `score-calculator.html`
- Place in `tools/` directory
- Add to `tools/README.md`

## Testing Checklist

- [ ] Works in Chrome
- [ ] Works in Firefox
- [ ] Works in Safari
- [ ] Works on mobile
- [ ] No console errors
- [ ] All interactions work
