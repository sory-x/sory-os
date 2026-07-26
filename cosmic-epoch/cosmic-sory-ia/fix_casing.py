#!/usr/bin/env python3
"""
Script de correction de la casse après renommage partiel OpenAI Codex → Sory IA.

Ce script corrige les imports et noms de fonctions qui utilisent encore la casse
"Sory_*" au lieu de la casse correcte "sory_*" (snake_case standard Rust).

Il cible spécifiquement les erreurs de compilation dans sory-rs/core/src/sory_delegate.rs
et les fichiers qui l'importent.
"""

import re
import sys
from pathlib import Path

# Mappings de correction : ancienne casse → nouvelle casse
CORRECTIONS = {
    # Imports de crates
    r'\bSory_analytics\b': 'sory_analytics',
    r'\bSory_async_utils\b': 'sory_async_utils',
    r'\bSory_protocol\b': 'sory_protocol',
    r'\bSory_login\b': 'sory_login',
    r'\bSory_models_manager\b': 'sory_models_manager',
    r'\bSory_rollout_trace\b': 'sory_rollout_trace',
    # Noms de fonctions
    r'\brun_Sory_thread_interactive\b': 'run_sory_thread_interactive',
    r'\brun_Sory_thread_one_shot\b': 'run_sory_thread_one_shot',
    # Types dans les paths
    r'\bSory_protocol::protocol::\b': 'sory_protocol::protocol::',
    r'\bSory_protocol::request_permissions::\b': 'sory_protocol::request_permissions::',
    r'\bSory_protocol::request_user_input::\b': 'sory_protocol::request_user_input::',
    r'\bSory_protocol::user_input::\b': 'sory_protocol::user_input::',
    r'\bSory_protocol::error::\b': 'sory_protocol::error::',
    r'\bSory_rollout_trace::ThreadTraceContext\b': 'sory_rollout_trace::ThreadTraceContext',
}

# Fichiers à corriger
TARGET_FILES = [
    'sory-ia/sory-rs/core/src/sory_delegate.rs',
    'sory-ia/sory-rs/core/src/guardian/review_session.rs',
    'sory-ia/sory-rs/core/src/tasks/review.rs',
]

def fix_file(filepath: Path):
    """Corrige la casse dans un fichier donné."""
    content = filepath.read_text(encoding='utf-8')
    original = content
    
    for pattern, replacement in CORRECTIONS.items():
        content = re.sub(pattern, replacement, content)
    
    if content != original:
        filepath.write_text(content, encoding='utf-8')
        print(f"✅ Corrigé : {filepath}")
        return True
    return False

def main():
    base_dir = Path(__file__).parent
    fixed_count = 0
    
    for target in TARGET_FILES:
        path = base_dir / target
        if path.exists():
            if fix_file(path):
                fixed_count += 1
        else:
            print(f"⚠️  Fichier introuvable : {path}")
    
    print(f"\n📊 Résumé : {fixed_count} fichier(s) corrigé(s)")
    
    if fixed_count > 0:
        print("\n🔧 Vous pouvez maintenant relancer la compilation avec :")
        print("   cd soryos/cosmic-epoch/cosmic-sory-ia/sory-ia/sory-rs")
        print("   cargo build -p sory-core")
        return 0
    else:
        print("\n❌ Aucun changement nécessaire ou fichiers introuvables.")
        return 1

if __name__ == '__main__':
    sys.exit(main())
