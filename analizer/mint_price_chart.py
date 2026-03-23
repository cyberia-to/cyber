# ---
# tags: cyber, python
# crystal-type: source
# crystal-domain: cyber
# ---
"""
Mint price chart for $V and $A tokens on Bostrom.

Shows how the cost of minting 1 unit of $V or $A (in $H) grows
as cumulative supply increases, driven by exponential supply decay.

Formula: finalMint = (H / baseAmount) × cycles × halving × 1000 × supplyDecay
  where supplyDecay = 0.5 ^ (totalSupply / halfLife)

Inverted: H per unit = baseAmount / (cycles × halving × 1000 × supplyDecay)

cycles × halving ≈ 33 (approximately constant, see tokenomics.md)
"""

import numpy as np
import matplotlib.pyplot as plt

# Parameters from go-cyber x/resources
HALF_LIFE_V = 4_000_000_000
HALF_LIFE_A = 32_000_000_000
BASE_AMOUNT_V = 1_000_000_000  # H per 1 base unit of V
BASE_AMOUNT_A = 100_000_000    # H per 1 base unit of A
CYCLES_X_HALVING = 33.0        # approximately constant product

# Current circulating supply from Bostrom LCD (Feb 2026, block ~22.8M)
# Note: totalSupply in decay formula includes burned V — circulating is a lower bound
CURRENT_SUPPLY_V = 2_182_354_162    # millivolt
CURRENT_SUPPLY_A = 13_886_792_950   # milliampere


def h_per_unit(total_supply, half_life, base_amount):
    """Cost in H to mint 1000 milli-units (= 1 unit) at given cumulative supply."""
    supply_decay = 0.5 ** (total_supply / half_life)
    return base_amount / (CYCLES_X_HALVING * supply_decay)


# Supply ranges
supply_v = np.linspace(0, HALF_LIFE_V * 4, 1000)
supply_a = np.linspace(0, HALF_LIFE_A * 4, 1000)

price_v = np.array([h_per_unit(s, HALF_LIFE_V, BASE_AMOUNT_V) for s in supply_v])
price_a = np.array([h_per_unit(s, HALF_LIFE_A, BASE_AMOUNT_A) for s in supply_a])

# Current prices
cur_price_v = h_per_unit(CURRENT_SUPPLY_V, HALF_LIFE_V, BASE_AMOUNT_V)
cur_price_a = h_per_unit(CURRENT_SUPPLY_A, HALF_LIFE_A, BASE_AMOUNT_A)

# --- Dark theme ---
plt.style.use('dark_background')
fig, (ax1, ax2) = plt.subplots(1, 2, figsize=(16, 7.5))
fig.patch.set_facecolor('#0d1117')
for ax in (ax1, ax2):
    ax.set_facecolor('#0d1117')

# --- $V chart ---
ax1.plot(supply_v / 1e9, price_v / 1e6, color='#FF6B35', linewidth=2.5)
ax1.axvline(x=CURRENT_SUPPLY_V / 1e9, color='#FF6B35', linestyle='--', alpha=0.6)
ax1.plot(CURRENT_SUPPLY_V / 1e9, cur_price_v / 1e6, 'o', color='#FF6B35', markersize=8, zorder=5)
ax1.annotate(f'now: {cur_price_v/1e6:.1f}M H\n({CURRENT_SUPPLY_V/1e9:.1f}B mV)',
             xy=(CURRENT_SUPPLY_V / 1e9, cur_price_v / 1e6),
             xytext=(CURRENT_SUPPLY_V / 1e9 + 1.5, cur_price_v / 1e6 + 30),
             fontsize=10, color='#FF6B35',
             arrowprops=dict(arrowstyle='->', color='#FF6B35', alpha=0.6))
ax1.set_xlabel('Cumulative $V supply (billions millivolt)', fontsize=12, color='#8b949e')
ax1.set_ylabel('Cost to mint 1 V (millions $H)', fontsize=12, color='#8b949e')
ax1.set_title('$V (VOLT)', fontsize=15, fontweight='bold', color='#FF6B35', pad=12)
ax1.grid(True, alpha=0.15, color='#30363d')
ax1.set_xlim(0, HALF_LIFE_V * 4 / 1e9)
ax1.tick_params(colors='#8b949e')

for i in range(1, 5):
    x = HALF_LIFE_V * i / 1e9
    ax1.axvline(x=x, color='#30363d', linestyle=':', alpha=0.5)
    ax1.text(x, ax1.get_ylim()[1] * 0.95, f'{i}× halfLife',
             rotation=90, va='top', ha='right', fontsize=8, color='#484f58')

# --- $A chart ---
ax2.plot(supply_a / 1e9, price_a / 1e6, color='#4ECDC4', linewidth=2.5)
ax2.axvline(x=CURRENT_SUPPLY_A / 1e9, color='#4ECDC4', linestyle='--', alpha=0.6)
ax2.plot(CURRENT_SUPPLY_A / 1e9, cur_price_a / 1e6, 'o', color='#4ECDC4', markersize=8, zorder=5)
ax2.annotate(f'now: {cur_price_a/1e6:.2f}M H\n({CURRENT_SUPPLY_A/1e9:.1f}B mA)',
             xy=(CURRENT_SUPPLY_A / 1e9, cur_price_a / 1e6),
             xytext=(CURRENT_SUPPLY_A / 1e9 + 12, cur_price_a / 1e6 + 3),
             fontsize=10, color='#4ECDC4',
             arrowprops=dict(arrowstyle='->', color='#4ECDC4', alpha=0.6))
ax2.set_xlabel('Cumulative $A supply (billions milliampere)', fontsize=12, color='#8b949e')
ax2.set_ylabel('Cost to mint 1 A (millions $H)', fontsize=12, color='#8b949e')
ax2.set_title('$A (AMPERE)', fontsize=15, fontweight='bold', color='#4ECDC4', pad=12)
ax2.grid(True, alpha=0.15, color='#30363d')
ax2.set_xlim(0, HALF_LIFE_A * 4 / 1e9)
ax2.tick_params(colors='#8b949e')

for i in range(1, 5):
    x = HALF_LIFE_A * i / 1e9
    ax2.axvline(x=x, color='#30363d', linestyle=':', alpha=0.5)
    ax2.text(x, ax2.get_ylim()[1] * 0.95, f'{i}× halfLife',
             rotation=90, va='top', ha='right', fontsize=8, color='#484f58')

# --- Subtitle ---
fig.text(0.5, 0.01,
         'supplyDecay = 0.5 ^ (totalSupply / halfLife)  ·  each halfLife doubles the mint price  ·  $V halfLife = 4B  ·  $A halfLife = 32B',
         ha='center', fontsize=10, color='#484f58')

plt.tight_layout(rect=[0, 0.04, 1, 1])
plt.savefig('/Users/joyrocket/git/cyber/media/mint_price_chart.png', dpi=150, bbox_inches='tight',
            facecolor='#0d1117', edgecolor='none')
print(f"Saved: media/mint_price_chart.png")
print(f"Current $V price: {cur_price_v/1e6:.1f}M H per 1 V")
print(f"Current $A price: {cur_price_a/1e6:.2f}M H per 1 A")
