# Contribuer à CESIZen API

## Workflow GitFlow

Ce projet suit la méthodologie **GitFlow**. Voici les règles à respecter.

### Branches permanentes

| Branche | Rôle | Protection |
|---------|------|-----------|
| `main` | Production — code stable et déployé | PR obligatoire, push direct interdit |
| `develop` | Intégration — branche de référence pour le développement | PR obligatoire, push direct interdit |

### Branches temporaires

| Type | Nommage | Base | Merge vers |
|------|---------|------|-----------|
| Nouvelle fonctionnalité | `feature/nom-court` | `develop` | `develop` |
| Préparation de release | `release/vX.Y.Z` | `develop` | `main` + `develop` |
| Correctif urgent | `hotfix/nom-court` | `main` | `main` + `develop` |

### Flux de travail

```
# 1. Créer une branche feature depuis develop
git checkout develop
git pull origin develop
git checkout -b feature/ma-fonctionnalite

# 2. Développer, commiter
git add .
git commit -m "feat(scope): description courte"

# 3. Pousser et ouvrir une PR vers develop
git push origin feature/ma-fonctionnalite
# → Ouvrir une PR sur GitHub

# 4. Après merge, supprimer la branche locale
git branch -d feature/ma-fonctionnalite
```

## Convention de commits

Ce projet suit [Conventional Commits](https://www.conventionalcommits.org/) :

```
<type>(<scope>): <description courte>
```

| Type | Usage |
|------|-------|
| `feat` | Nouvelle fonctionnalité |
| `fix` | Correction de bug |
| `chore` | Maintenance, dépendances, CI |
| `refactor` | Refactorisation sans changement de comportement |
| `test` | Ajout ou modification de tests |
| `docs` | Documentation uniquement |

## Versioning sémantique

Les releases suivent [SemVer](https://semver.org/) : `vMAJEUR.MINEUR.PATCH`

- `MAJEUR` : changement incompatible avec l'existant
- `MINEUR` : nouvelle fonctionnalité rétrocompatible
- `PATCH` : correction de bug rétrocompatible

## Pull Requests

- Toute PR doit remplir le template fourni
- Lier la PR à une issue avec `Closes #xxx`
- Les tests doivent passer avant le merge (`cargo test`)
