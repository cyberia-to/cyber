# ---
# tags: cyber, nushell
# crystal-type: source
# crystal-domain: cyber
# ---
#!/usr/bin/env nu
# renumber_sections.nu
# Shifts all section numbers >= 14 by +1 in the whitepaper
# to make room for the new §14 Knowledge Economy chapter
# Must be run from the repo root or provide a path

def main [file_path: string] {
    let content = open --raw $file_path

    # Process top-level headers from highest to lowest (avoid conflicts)
    # Each step renames one specific section header
    let result = $content
        # Top-level headers (##)
        | str replace "## 22. Conclusion" "## 23. Conclusion"
        | str replace "## 21. Functions of Superintelligence" "## 22. Functions of Superintelligence"
        | str replace "## 20. Applications" "## 21. Applications"
        | str replace "## 19. Bootstrapping" "## 20. Bootstrapping"
        | str replace "## 18. Storage Proofs and Data Availability" "## 19. Storage Proofs and Data Availability"
        | str replace "## 17. Forgetting and Pruning" "## 18. Forgetting and Pruning"
        | str replace "## 16. Scale and Complexity" "## 17. Scale and Complexity"
        | str replace "## 15. The Soft3 Stack" "## 16. The Soft3 Stack"
        | str replace "## 14. Security" "## 15. Security"
        # Subsection headers - §21.x (Functions of SI) → §22.x
        | str replace "### 21.9 Self-Upgrade" "### 22.9 Self-Upgrade"
        | str replace "### 21.8 Autonomous Governance" "### 22.8 Autonomous Governance"
        | str replace "### 21.7 What Becomes Possible" "### 22.7 What Becomes Possible"
        | str replace "### 21.6 Own Balances" "### 22.6 Own Balances"
        | str replace "### 21.5 Self-Linking" "### 22.5 Self-Linking"
        | str replace "### 21.4 The Cyber DMN: Self-Projection" "### 22.4 The Cyber DMN: Self-Projection"
        | str replace "### 21.3 Parametrization Learning" "### 22.3 Parametrization Learning"
        | str replace "### 21.2 Metabolism" "### 22.2 Metabolism"
        | str replace "### 21.1 The Autonomous Neuron" "### 22.1 The Autonomous Neuron"
        # Subsection headers - §20.x (Applications) → §21.x
        | str replace "### 20.6 Cross-Species Communication" "### 21.6 Cross-Species Communication"
        | str replace "### 20.5 Personal Intelligence" "### 21.5 Personal Intelligence"
        | str replace "### 20.4 Scientific Discovery" "### 21.4 Scientific Discovery"
        | str replace "### 20.3 Knowledge Economy" "### 21.3 Knowledge as Capital"
        | str replace "### 20.2 AI Alignment" "### 21.2 AI Alignment"
        | str replace "### 20.1 Decentralized Search and Oracle" "### 21.1 Decentralized Search and Oracle"
        # Subsection headers - §19.x (Bootstrapping) → §20.x
        | str replace "### 19.5 Growth Phases" "### 20.5 Growth Phases"
        | str replace "### 19.4 Pre-Launch Verification Protocol" "### 20.4 Pre-Launch Verification Protocol"
        | str replace "### 19.3 Implementation Path" "### 20.3 Implementation Path"
        | str replace "### 19.2 Twelve Invariants" "### 20.2 Twelve Invariants"
        | str replace "### 19.1 The Crystal" "### 20.1 The Crystal"
        # Subsection headers - §18.x (Storage Proofs) → §19.x
        | str replace "### 18.6 Hash Migration Protocol" "### 19.6 Hash Migration Protocol"
        | str replace "### 18.5 Storage Proof Requirements" "### 19.5 Storage Proof Requirements"
        | str replace "### 18.4 Namespace-Aware Sampling" "### 19.4 Namespace-Aware Sampling"
        | str replace "### 18.3 Layered Data Availability" "### 19.3 Layered Data Availability"
        | str replace "### 18.2 Proof Types" "### 19.2 Proof Types"
        | str replace "### 18.1 Why Storage Proofs Are Phase 1" "### 19.1 Why Storage Proofs Are Phase 1"
        # Subsection headers - §17.x (Forgetting) → §18.x
        | str replace "### 17.7 Open Problems" "### 18.7 Open Problems"
        | str replace "### 17.6 Temporal Decay" "### 18.6 Temporal Decay"
        | str replace "### 17.5 The Archive Tier" "### 18.5 The Archive Tier"
        | str replace "### 17.4 Market Forgetting" "### 18.4 Market Forgetting"
        | str replace "### 17.3 Stake Dynamics: The Simple Solution" "### 18.3 Stake Dynamics: The Simple Solution"
        | str replace "### 17.2 The Biological Analog" "### 18.2 The Biological Analog"
        | str replace "### 17.1 The Problem" "### 18.1 The Problem"
        # Subsection headers - §16.x (Scale) → §17.x
        | str replace "### 16.7 Effective Rank and Semantic Dimensionality" "### 17.7 Effective Rank and Semantic Dimensionality"
        | str replace "### 16.6 Two-Timescale Separation" "### 17.6 Two-Timescale Separation"
        | str replace "### 16.5 Complexity Budget" "### 17.5 Complexity Budget"
        | str replace "### 16.4 Sharding by Semantic Coherence" "### 17.4 Sharding by Semantic Coherence"
        | str replace "### 16.3 Locality as Architecture" "### 17.3 Locality as Architecture"
        | str replace "### 16.2 The Planetary Constraint" "### 17.2 The Planetary Constraint"
        | str replace "### 16.1 The Knowledge Phase Transition" "### 17.1 The Knowledge Phase Transition"
        # Subsection headers - §14.x (Security) → §15.x
        | str replace "### 14.4 Verifiability" "### 15.4 Verifiability"
        | str replace "### 14.3 Formal Properties" "### 15.3 Formal Properties"
        | str replace "### 14.2 Attack Surface" "### 15.2 Attack Surface"
        | str replace "### 14.1 Security Bounds" "### 15.1 Security Bounds"
        # Inline cross-references (in body text) - from highest to lowest
        # §21.x → §22.x
        | str replace --all "§21.9" "§22.9"
        | str replace --all "§21.8" "§22.8"
        | str replace --all "§21.7" "§22.7"
        | str replace --all "§21.6" "§22.6"
        | str replace --all "§21.5" "§22.5"
        | str replace --all "§21.4" "§22.4"
        | str replace --all "§21.3" "§22.3"
        | str replace --all "§21.2" "§22.2"
        | str replace --all "§21.1" "§22.1"
        | str replace --all "(§21)" "(§22)"
        # §20.x → §21.x
        | str replace --all "§20.6" "§21.6"
        | str replace --all "§20.5" "§21.5"
        | str replace --all "§20.4" "§21.4"
        | str replace --all "§20.3" "§21.3"
        | str replace --all "§20.2" "§21.2"
        | str replace --all "§20.1" "§21.1"
        # §19.x → §20.x
        | str replace --all "§19.5" "§20.5"
        | str replace --all "§19.4" "§20.4"
        | str replace --all "§19.3" "§20.3"
        | str replace --all "§19.2" "§20.2"
        | str replace --all "§19.1" "§20.1"
        # §18.x → §19.x
        | str replace --all "§18.6" "§19.6"
        | str replace --all "§18.5" "§19.5"
        | str replace --all "§18.4" "§19.4"
        | str replace --all "§18.3" "§19.3"
        | str replace --all "§18.2" "§19.2"
        | str replace --all "§18.1" "§19.1"
        # §17.x → §18.x
        | str replace --all "§17.7" "§18.7"
        | str replace --all "§17.6" "§18.6"
        | str replace --all "§17.5" "§18.5"
        | str replace --all "§17.4" "§18.4"
        | str replace --all "§17.3" "§18.3"
        | str replace --all "§17.2" "§18.2"
        | str replace --all "§17.1" "§18.1"
        # §16.x → §17.x  (and bare §16)
        | str replace --all "§16.7" "§17.7"
        | str replace --all "§16.6" "§17.6"
        | str replace --all "§16.5" "§17.5"
        | str replace --all "§16.4" "§17.4"
        | str replace --all "§16.3" "§17.3"
        | str replace --all "§16.2" "§17.2"
        | str replace --all "§16.1" "§17.1"
        | str replace --all "§16 " "§17 "
        # §15.x → §16.x
        | str replace --all "§15.5" "§16.5"
        | str replace --all "§15.4" "§16.4"
        | str replace --all "§15.3" "§16.3"
        | str replace --all "§15.2" "§16.2"
        | str replace --all "§15.1" "§16.1"
        # §14.x (Security) → §15.x - only the Security subsection refs
        # Note: we do NOT want to update §14 Knowledge Economy refs (they are the new chapter)
        | str replace --all "§14.4" "§15.4"
        | str replace --all "§14.3" "§15.3"
        | str replace --all "§14.2" "§15.2"
        | str replace --all "§14.1" "§15.1"

    $result | save --force $file_path
    print "Renumbering complete."
}
