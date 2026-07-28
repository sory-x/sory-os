# Configuration des secrets GitHub

## 1. Exporter la clé GPG

```bash
export GNUPGHOME=/home/sory/Bureau/soryos/soryos-apt/.private/gnupg
gpg --armor --export-secret-keys apt@soryos.local
```

Copier tout le bloc dans le secret `SORYOS_GPG_PRIVATE_KEY`.

## 2. Créer un token GitHub

1. Aller sur https://github.com/settings/tokens
2. Cliquer **Generate new token (classic)**
3. Cocher `repo` (Full control of private repositories)
4. Copier la valeur dans le secret `APT_REPO_TOKEN`

## 3. Ajouter les secrets

1. Aller sur https://github.com/sory-x/sory-os/settings/secrets/actions
2. Ajouter `SORYOS_GPG_PRIVATE_KEY` et `APT_REPO_TOKEN`

## 4. Pousser le monorepo

```bash
cd /home/sory/Bureau/soryos
git push origin main
```
