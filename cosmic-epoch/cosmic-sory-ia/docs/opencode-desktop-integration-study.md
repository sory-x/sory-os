# Étude OpenCode Desktop → pistes pour Sory IA (archive migration)

## Résumé

Le dossier `docs-compare-projet/opencode-dev` montre une architecture mature où le Desktop n’embarque pas de logique IA. Il démarre un serveur local isolé nommé *sidecar*, attend sa disponibilité, puis l’interface consomme une API HTTP + SSE via un SDK généré.

Pour Sory IA, le principe à reprendre n’est pas Electron/JS, mais le découpage :

```text
Desktop natif
  → RuntimeManager / SidecarManager
  → serveur/runtime local
  → API/protocole officiel
  → flux d’événements centralisé
  → état UI
```

Codex/Sory IA possède déjà l’équivalent bas niveau avec `app-server`, `app-server-daemon`, `app-server-client`, `app-server-protocol`. Il faut donc s’appuyer dessus au lieu d’inventer un protocole.

---

## Architecture OpenCode observée

### 1. Monorepo organisé par rôles

OpenCode sépare clairement :

- `packages/opencode` : CLI + runtime + serveur + providers + sessions + outils.
- `packages/desktop` : application desktop, lifecycle, fenêtres, IPC, sidecar.
- `packages/app` : interface partagée web/desktop.
- `packages/sdk/js` : client API généré depuis le contrat HTTP/OpenAPI.

Cette séparation est importante : le Desktop ne dépend pas directement des agents/providers/outils.

---

### 2. Démarrage Desktop

Dans `packages/desktop/src/main/index.ts`, le Desktop :

1. prépare l’environnement ;
2. force les exceptions proxy loopback ;
3. charge les certificats système ;
4. choisit un port libre ;
5. génère un mot de passe temporaire ;
6. démarre un sidecar ;
7. attend une notification `ready` ;
8. effectue un health check `/global/health` ;
9. expose `{ url, username, password }` au renderer.

Points clés à reprendre pour Sory IA :

- ne pas considérer le runtime prêt dès que le process est lancé ;
- attendre explicitement un signal prêt + health check ;
- gérer les logs stdout/stderr du runtime ;
- arrêter proprement le runtime à la fermeture ;
- protéger la connexion locale par auth ou socket privé.

---

### 3. Sidecar

Fichiers :

- `packages/desktop/src/main/server.ts`
- `packages/desktop/src/main/sidecar.ts`

OpenCode ne lance pas le CLI principal directement dans le renderer. Il lance un processus isolé via `utilityProcess.fork`.

Le sidecar :

- reçoit une commande `{ type: "start", hostname, port, password, userDataPath, needsMigration }` ;
- prépare l’environnement ;
- lance `Server.listen(...)` depuis le runtime ;
- annonce `ready` ;
- peut annoncer une progression SQLite ;
- s’arrête proprement sur `{ type: "stop" }`.

Pour Sory IA Rust natif, l’équivalent doit être :

```text
RuntimeManager
  start()
  wait_ready()
  health_check()
  stop()
  restart()
  collect_logs()
```

On n’a pas besoin d’un sidecar JS, mais on doit garder la même responsabilité isolée.

---

### 4. API HTTP + SSE

OpenCode expose :

- `/global/health`
- `/global/event` en SSE
- `/event` pour événements instance/workspace
- `/session`
- `/session/:id/message`
- `/session/:id/abort`
- `/session/:id/message/:messageID`
- etc.

Le Desktop n’observe pas directement le moteur IA. Il observe un flux d’événements normalisé.

La route importante :

```text
POST /session/:sessionID/message
```

Elle crée le message utilisateur et déclenche le traitement IA. La réponse peut retourner un message initial, mais le rendu temps réel se fait via SSE.

---

### 5. SDK généré

`packages/sdk/js` génère un client typé autour de l’API HTTP.

Le renderer ne construit pas manuellement les URLs partout. Il utilise :

```ts
createOpencodeClient({ baseUrl, headers, directory })
```

Pour Sory IA, l’équivalent Rust souhaitable est :

```text
backend/api.rs
  SoryIaApiClient
  health()
  create_session()
  send_message()
  abort_session()
  list_sessions()
  subscribe_events()
```

Même si on utilise le protocole Codex app-server actuel, il faut éviter que l’UI connaisse les détails JSON-RPC/WebSocket.

---

### 6. Synchronisation d’état

OpenCode a un `global-sync` très important.

Fichiers étudiés :

- `packages/app/src/context/global-sdk.tsx`
- `packages/app/src/context/global-sync.tsx`
- `packages/app/src/context/global-sync/event-reducer.ts`

Le flux est :

```text
SSE /global/event
  → queue d’événements
  → coalescing
  → heartbeat timeout
  → reconnect
  → reducer d’événements
  → stores UI
```

Événements observés :

- `server.connected`
- `global.disposed`
- `project.updated`
- `session.created`
- `session.updated`
- `session.deleted`
- `session.status`
- `message.updated`
- `message.removed`
- `message.part.updated`
- `message.part.delta`
- `permission.asked`
- `permission.replied`
- `question.asked`
- `todo.updated`

Le streaming texte n’est pas un simple append brut dans le composant chat. Il arrive comme événements de parties de message :

```text
message.part.delta
message.part.updated
message.updated
session.status
```

Pour Sory IA, l’état Desktop devrait évoluer vers :

```text
BackendEvent
  → EventQueue
  → reducer centralisé
  → ConversationState / MessageState / ToolState / PermissionState
  → UI passive
```

---

## Comparaison avec Sory IA actuel

### Ce qui va dans le bon sens

Le Desktop Sory IA possède déjà :

- `backend/`
- `RuntimeManager`
- `BackendClient`
- `BackendConnection`
- `BackendCommand`
- `BackendEvent`
- état centralisé basique
- UI passive basique

La décision récente d’utiliser `codex-app-server-client` / `codex-app-server-protocol` est cohérente avec OpenCode : réutiliser le protocole runtime officiel.

### Ce qui manque encore

1. **Ready/health explicite**
   - OpenCode attend `ready` puis `/global/health`.
   - Sory IA doit attendre une vraie connexion app-server initialisée, pas seulement l’existence du socket.

2. **Session runtime persistée dans l’état**
   - OpenCode a des sessions côté runtime et l’UI affiche ces sessions.
   - Sory IA ne doit pas créer un UUID Desktop comme source principale ; il doit mapper `Conversation` ↔ `thread_id/session_id` du runtime.

3. **Event reducer centralisé**
   - OpenCode a un reducer robuste.
   - Sory IA doit éviter de traiter les événements directement dans `app/mod.rs` à long terme.

4. **Queue/coalescing/heartbeat**
   - OpenCode coalesce certains événements pour éviter de saturer l’UI.
   - Sory IA doit prévoir une queue backend → state, surtout pour tokens rapides.

5. **Abstraction API stable**
   - OpenCode a un SDK.
   - Sory IA doit avoir un `SoryIaRuntimeClient` interne, même si dessous c’est JSON-RPC/WebSocket/UDS Codex.

6. **Gestion permissions/questions**
   - OpenCode expose permission/question comme événements UI.
   - Sory IA doit mapper les `ServerRequest` Codex/app-server vers `PermissionRequested`, `QuestionAsked`, etc.

---

## Architecture cible recommandée pour Sory IA

```text
sory-desktop/src/backend/
  mod.rs
  client.rs          BackendClient public
  runtime.rs         RuntimeManager process/daemon
  connection.rs      connexion app-server officielle
  protocol.rs        BackendCommand / BackendEvent normalisés
  event_queue.rs     buffer, coalescing, heartbeat, reconnect
  reducer.rs         mapping BackendEvent → StatePatch ou AppEvent
  session.rs         mapping ConversationId ↔ runtime thread/session id
  error.rs
```

Flux :

```text
UI native libcosmic
  ↓ AppEvent
ApplicationState reducer
  ↓ BackendCommand
BackendClient
  ↓
RuntimeManager démarre le CLI/app-server si nécessaire
  ↓
BackendConnection parle au protocole officiel Codex/Sory IA
  ↓
EventQueue reçoit streaming / tool / permissions
  ↓
State reducer applique les événements
  ↓
UI observe l’état
```

---

## Règles d’implémentation pour la suite

1. Ne pas déplacer la logique IA dans Desktop.
2. Ne pas créer un protocole parallèle si `app-server-protocol` couvre le besoin.
3. Garder `codex-main` synchronisable.
4. Mapper les concepts visibles en `Sory IA`, mais conserver les crates/protocoles externes inchangés.
5. Ne pas exposer le mot `Codex` dans l’UI.
6. Privilégier socket local/UDS ou app-server officiel plutôt que stdio brut.
7. Ajouter un health check et une phase `Initializing / Ready / Reconnecting / Failed`.
8. Centraliser tous les événements dans un reducer.
9. Garder une table de correspondance :

```text
Conversation.id      UUID Desktop stable
Conversation.thread  ID runtime app-server
Message.id           ID Desktop ou runtime selon disponibilité
Message.runtime_id   ID message/part runtime
```

---

## Conclusion

OpenCode confirme que la bonne architecture n’est pas :

```text
Desktop → CLI texte → parsing stdout
```

mais :

```text
Desktop → runtime local contrôlé → API/protocole officiel → flux événementiel → état centralisé
```

Pour Sory IA, l’équivalent naturel est le `codex app-server daemon` / `codex-app-server-protocol` déjà présent dans `codex-rs`. Le travail doit maintenant consister à rendre notre `backend/` aussi robuste que le sidecar OpenCode : démarrage fiable, health check, reconnexion, queue d’événements, mapping sessions runtime, reducer centralisé et intégration UI passive.
