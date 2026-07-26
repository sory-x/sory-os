# Sory IA Desktop

Sory IA Desktop est le client graphique natif SoryOS pour piloter le moteur IA du projet Sory IA.

## Principes

- L'expérience utilisateur, le branding, les fenêtres, la documentation et les textes affichés utilisent **Sory IA**.
- Le moteur IA reste encapsulé derrière `src/backend` pour conserver la compatibilité avec le runtime existant.
- La communication avec le runtime est asynchrone via Tokio.
- La UI ne communique jamais directement avec le moteur : tout passe par `BackendClient` et les événements applicatifs.
- Les dépendances externes, protocoles et APIs tierces ne sont pas renommés.

## Architecture

```text
src/
  app/          Démarrage et orchestration libcosmic
  ui/           Composition visuelle sans logique métier
  backend/      Client unique et transport isolé vers le runtime IA
  state/        État applicatif centralisé
  models/       Structures de données Sory IA
  events/       Événements internes
  components/   Widgets réutilisables
  theme/        Adaptation Design System SoryOS
  icons/        Noms et chargement d’icônes
  pages/        Écrans
  platform/     Abstraction des services SoryOS
```

## Backend

`BackendClient` est la seule API que l'application doit utiliser pour :

- démarrer le daemon app-server exposé par le CLI si nécessaire ;
- connecter / déconnecter le runtime via le socket Unix officiel ;
- reconnecter automatiquement ;
- envoyer les requêtes ;
- recevoir le streaming ;
- remonter les erreurs ;
- surveiller le processus.

L'intégration réutilise les crates/protocoles app-server existants du runtime au lieu d'inventer une nouvelle couche de communication.

La commande du CLI/runtime peut être configurée avec :

```sh
SORY_IA_RUNTIME_COMMAND="sory"
```

La valeur par défaut reste `sory` pour préserver la compatibilité technique avec le moteur existant, mais cette information ne doit pas être exposée comme nom produit à l'utilisateur final.
