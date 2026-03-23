# ---
# tags: cyber, python
# crystal-type: source
# crystal-domain: cyber
# ---
"""
Individual token charts for $BOOT, $H, $V, $A pages.
Generates 4 charts uploaded to IPFS via Pinata.
"""

import numpy as np
import matplotlib.pyplot as plt
import subprocess, json, os

# Live chain data (Feb 2026, block ~22.8M)
BOOT_SUPPLY = 480_342_288_047_835
H_SUPPLY = 297_359_046_515_772
V_SUPPLY = 2_182_354_162          # millivolt
A_SUPPLY = 13_886_792_950         # milliampere
BONDED = 259_587_578_615_162
NOT_BONDED = 47_779_260_830_183
INFLATION = 0.0109
BLOCK = 22_805_234

# Mint parameters
HALF_LIFE_V = 4_000_000_000
HALF_LIFE_A = 32_000_000_000
BASE_AMOUNT_V = 1_000_000_000
BASE_AMOUNT_A = 100_000_000

MEDIA_DIR = os.path.join(os.path.dirname(os.path.dirname(os.path.abspath(__file__))), 'media')
os.makedirs(MEDIA_DIR, exist_ok=True)

# Dark theme
plt.style.use('dark_background')
BG = '#0d1117'
GRID = '#30363d'
TEXT = '#8b949e'

def save(fig, name):
    path = os.path.join(MEDIA_DIR, name)
    fig.savefig(path, dpi=150, bbox_inches='tight', facecolor=BG, edgecolor='none')
    plt.close(fig)
    print(f'Saved: {path}')
    return path


# --- $V chart: mint price curve ---
def chart_v():
    fig, ax = plt.subplots(figsize=(10, 6))
    fig.patch.set_facecolor(BG)
    ax.set_facecolor(BG)

    supply = np.linspace(0, HALF_LIFE_V * 4, 1000)
    price = BASE_AMOUNT_V / (0.5 ** (supply / HALF_LIFE_V))
    cur_price = BASE_AMOUNT_V / (0.5 ** (V_SUPPLY / HALF_LIFE_V))

    ax.plot(supply / 1e9, price / 1e6, color='#FF6B35', linewidth=2.5)
    ax.plot(V_SUPPLY / 1e9, cur_price / 1e6, 'o', color='#FF6B35', markersize=10, zorder=5)
    max_price = price[-1] / 1e6
    ax.annotate(f'now: {cur_price/1e6:.1f}M H per V\nsupply: {V_SUPPLY/1e9:.1f}B mV',
                xy=(V_SUPPLY / 1e9, cur_price / 1e6),
                xytext=(HALF_LIFE_V * 1.5 / 1e9, max_price * 0.55),
                fontsize=11, color='#FF6B35',
                arrowprops=dict(arrowstyle='->', color='#FF6B35', alpha=0.6))

    for i in range(1, 5):
        ax.axvline(x=HALF_LIFE_V * i / 1e9, color=GRID, linestyle=':', alpha=0.5)

    ax.set_xlabel('Cumulative supply (billions millivolt)', fontsize=12, color=TEXT)
    ax.set_ylabel('Price per 1 V (millions H)', fontsize=12, color=TEXT)
    ax.set_title('$V mint price — supply decay curve', fontsize=14, fontweight='bold', color='#FF6B35')
    ax.tick_params(colors=TEXT)
    ax.grid(True, alpha=0.15, color=GRID)
    fig.text(0.5, 0.01, f'halfLife = 4B  ·  price doubles every 4B millivolt minted  ·  block {BLOCK:,}',
             ha='center', fontsize=9, color='#484f58')
    plt.tight_layout(rect=[0, 0.04, 1, 1])
    return save(fig, 'chart_v.png')


# --- $A chart: mint price curve ---
def chart_a():
    fig, ax = plt.subplots(figsize=(10, 6))
    fig.patch.set_facecolor(BG)
    ax.set_facecolor(BG)

    supply = np.linspace(0, HALF_LIFE_A * 4, 1000)
    price = BASE_AMOUNT_A / (0.5 ** (supply / HALF_LIFE_A))
    cur_price = BASE_AMOUNT_A / (0.5 ** (A_SUPPLY / HALF_LIFE_A))

    ax.plot(supply / 1e9, price / 1e6, color='#4ECDC4', linewidth=2.5)
    ax.plot(A_SUPPLY / 1e9, cur_price / 1e6, 'o', color='#4ECDC4', markersize=10, zorder=5)
    max_price = price[-1] / 1e6
    ax.annotate(f'now: {cur_price/1e6:.0f}M H per A\nsupply: {A_SUPPLY/1e9:.1f}B mA',
                xy=(A_SUPPLY / 1e9, cur_price / 1e6),
                xytext=(HALF_LIFE_A * 1.5 / 1e9, max_price * 0.55),
                fontsize=11, color='#4ECDC4',
                arrowprops=dict(arrowstyle='->', color='#4ECDC4', alpha=0.6))

    for i in range(1, 5):
        ax.axvline(x=HALF_LIFE_A * i / 1e9, color=GRID, linestyle=':', alpha=0.5)

    ax.set_xlabel('Cumulative supply (billions milliampere)', fontsize=12, color=TEXT)
    ax.set_ylabel('Price per 1 A (millions H)', fontsize=12, color=TEXT)
    ax.set_title('$A mint price — supply decay curve', fontsize=14, fontweight='bold', color='#4ECDC4')
    ax.tick_params(colors=TEXT)
    ax.grid(True, alpha=0.15, color=GRID)
    fig.text(0.5, 0.01, f'halfLife = 32B  ·  price doubles every 32B milliampere minted  ·  block {BLOCK:,}',
             ha='center', fontsize=9, color='#484f58')
    plt.tight_layout(rect=[0, 0.04, 1, 1])
    return save(fig, 'chart_a.png')


# --- $H chart: where H goes (burned on V, burned on A, liquid) ---
def chart_h():
    import math
    fig, ax = plt.subplots(figsize=(10, 6))
    fig.patch.set_facecolor(BG)
    ax.set_facecolor(BG)

    ln2 = math.log(2)
    # H burned = integral of price per milliunit from 0 to totalSupply
    h_burned_v = 1_000_000 * HALF_LIFE_V / ln2 * (2**(V_SUPPLY / HALF_LIFE_V) - 1)
    h_burned_a = 100_000 * HALF_LIFE_A / ln2 * (2**(A_SUPPLY / HALF_LIFE_A) - 1)

    labels = [f'Burned on $V mint\n{h_burned_v/1e12:.0f}T',
              f'Burned on $A mint\n{h_burned_a/1e12:.0f}T',
              f'Liquid $H\n{H_SUPPLY/1e12:.0f}T']
    sizes = [h_burned_v, h_burned_a, H_SUPPLY]
    colors_list = ['#FF6B35', '#4ECDC4', '#58A6FF']
    explode = (0.03, 0.03, 0.05)

    wedges, texts, autotexts = ax.pie(sizes, labels=labels, colors=colors_list,
                                       explode=explode, autopct='%1.0f%%',
                                       textprops={'color': TEXT, 'fontsize': 11},
                                       pctdistance=0.65)
    for t in autotexts:
        t.set_color('white')
        t.set_fontsize(12)

    total = h_burned_v + h_burned_a + H_SUPPLY
    ax.set_title('Where $H goes', fontsize=14, fontweight='bold', color='#58A6FF', pad=15)
    fig.text(0.5, 0.02,
             f'Total $H ever created: {total/1e12:.0f}T  ·  {(h_burned_v+h_burned_a)/total*100:.0f}% permanently burned  ·  block {BLOCK:,}',
             ha='center', fontsize=9, color='#484f58')
    plt.tight_layout(rect=[0, 0.05, 1, 1])
    return save(fig, 'chart_h.png')


# --- $BOOT chart: staking ratio ---
def chart_boot():
    fig, ax = plt.subplots(figsize=(10, 6))
    fig.patch.set_facecolor(BG)
    ax.set_facecolor(BG)

    total = BOOT_SUPPLY
    bonded_pct = BONDED / total * 100
    liquid_pct = 100 - bonded_pct

    labels = [f'Bonded\n{bonded_pct:.1f}%', f'Liquid\n{liquid_pct:.1f}%']
    sizes = [bonded_pct, liquid_pct]
    colors_list = ['#A371F7', '#30363d']
    explode = (0.05, 0)

    wedges, texts, autotexts = ax.pie(sizes, labels=labels, colors=colors_list,
                                       explode=explode, autopct='',
                                       textprops={'color': TEXT, 'fontsize': 12})

    ax.set_title('$BOOT staking distribution', fontsize=14, fontweight='bold', color='#A371F7', pad=15)
    fig.text(0.5, 0.02,
             f'Total: {total/1e12:.0f}T  ·  Inflation: {INFLATION*100:.2f}%  ·  Target bonded: 25.49%  ·  block {BLOCK:,}',
             ha='center', fontsize=9, color='#484f58')
    plt.tight_layout(rect=[0, 0.05, 1, 1])
    return save(fig, 'chart_boot.png')


if __name__ == '__main__':
    chart_v()
    chart_a()
    chart_h()
    chart_boot()
    print('All charts generated.')
