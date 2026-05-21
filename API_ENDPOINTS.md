# CESIZen API - Liste des Endpoints

Documentation des endpoints API a implementer pour le projet CESIZen.

---

## 1. Authentification

| Methode | Endpoint | Description |
|---------|----------|-------------|
| POST | `/api/auth/register` | Inscription utilisateur |
| POST | `/api/auth/login` | Connexion |
| POST | `/api/auth/logout` | Deconnexion |
| POST | `/api/auth/forgot-password` | Mot de passe oublie |
| POST | `/api/auth/reset-password` | Reinitialiser mot de passe |

---

## 2. Utilisateurs

| Methode | Endpoint | Description |
|---------|----------|-------------|
| GET | `/api/users/me` | Profil utilisateur courant |
| PUT | `/api/users/me` | Modifier profil |
| PUT | `/api/users/me/password` | Changer mot de passe |

---

## 3. Tracker d'emotions (utilisateur connecte)

| Methode | Endpoint | Description | Parametres |
|---------|----------|-------------|------------|
| GET | `/api/tracker/entries` | Liste des entrees | `?period=week/month/quarter/year&emotion_id=X&page=1&limit=10` |
| GET | `/api/tracker/entries/{id}` | Detail d'une entree | - |
| POST | `/api/tracker/entries` | Ajouter une entree | `{ emotion_id, sub_emotion_id, intensity, note, date }` |
| PUT | `/api/tracker/entries/{id}` | Modifier une entree | `{ emotion_id, sub_emotion_id, intensity, note, date }` |
| DELETE | `/api/tracker/entries/{id}` | Supprimer une entree | - |
| GET | `/api/tracker/stats` | Stats rapides | Entrees semaine, emotion dominante, intensite moyenne |

---

## 4. Rapports (utilisateur connecte)

| Methode | Endpoint | Description | Parametres |
|---------|----------|-------------|------------|
| GET | `/api/reports/emotions` | Rapport emotions | `?period=week/month/quarter/year` |
| GET | `/api/reports/distribution` | Distribution des emotions | `?period=week/month/quarter/year` |
| GET | `/api/reports/trends` | Tendances/evolution | `?period=week/month/quarter/year` |

---

## 5. Emotions (referentiel public)

| Methode | Endpoint | Description |
|---------|----------|-------------|
| GET | `/api/emotions` | Liste des emotions de base |
| GET | `/api/emotions/{id}/sub-emotions` | Sous-emotions d'une emotion |

### Referentiel des emotions de base

| Emotion | Emoji | Sous-emotions |
|---------|-------|---------------|
| Joie | 😊 | Fierte, Contentement, Enchantement, Excitation, Emerveillement, Gratitude |
| Colere | 😠 | Frustration, Irritation, Rage, Ressentiment, Agacement, Hostilite |
| Peur | 😨 | Inquietude, Anxiete, Terreur, Apprehension, Panique, Crainte |
| Tristesse | 😢 | Chagrin, Melancolie, Abattement, Desespoir, Solitude, Depression |
| Surprise | 😲 | Etonnement, Stupefaction, Sideration, Incredulite, Emerveillement, Confusion |
| Degout | 🤢 | Repulsion, Deplaisir, Nausee, Dedain, Horreur, Degout profond |

---

## 6. Informations / Contenus (acces public)

| Methode | Endpoint | Description | Parametres |
|---------|----------|-------------|------------|
| GET | `/api/information/pages` | Liste articles/pages | `?category=X&page=1&limit=10` |
| GET | `/api/information/pages/{id}` | Detail d'une page | - |

---

## 7. Administration - Utilisateurs

| Methode | Endpoint | Description | Parametres |
|---------|----------|-------------|------------|
| GET | `/api/admin/users` | Liste utilisateurs | `?page=1&limit=10&status=active/inactive&search=X` |
| GET | `/api/admin/users/{id}` | Detail utilisateur | - |
| PUT | `/api/admin/users/{id}` | Modifier utilisateur | `{ email, first_name, last_name, role }` |
| PATCH | `/api/admin/users/{id}/status` | Activer/desactiver compte | `{ status: "active" / "inactive" }` |
| DELETE | `/api/admin/users/{id}` | Supprimer utilisateur | - |
| POST | `/api/admin/users` | Creer admin | `{ email, password, first_name, last_name, role }` |

---

## 8. Administration - Contenus

| Methode | Endpoint | Description | Parametres |
|---------|----------|-------------|------------|
| GET | `/api/admin/contents` | Liste contenus | `?page=1&limit=10&status=published/draft` |
| GET | `/api/admin/contents/{id}` | Detail contenu | - |
| POST | `/api/admin/contents` | Creer contenu | `{ title, slug, content, category, status }` |
| PUT | `/api/admin/contents/{id}` | Modifier contenu | `{ title, slug, content, category, status }` |
| DELETE | `/api/admin/contents/{id}` | Supprimer contenu | - |

---

## 9. Administration - Emotions

| Methode | Endpoint | Description | Parametres |
|---------|----------|-------------|------------|
| GET | `/api/admin/emotions` | Liste emotions | - |
| POST | `/api/admin/emotions` | Creer emotion | `{ name, emoji, color }` |
| PUT | `/api/admin/emotions/{id}` | Modifier emotion | `{ name, emoji, color }` |
| DELETE | `/api/admin/emotions/{id}` | Supprimer emotion | - |
| POST | `/api/admin/emotions/{id}/sub-emotions` | Ajouter sous-emotion | `{ name }` |
| PUT | `/api/admin/sub-emotions/{id}` | Modifier sous-emotion | `{ name }` |
| DELETE | `/api/admin/sub-emotions/{id}` | Supprimer sous-emotion | - |

---

## 10. Administration - Dashboard

| Methode | Endpoint | Description | Parametres |
|---------|----------|-------------|------------|
| GET | `/api/admin/stats` | Stats globales | Users, entries, contenus |
| GET | `/api/admin/stats/registrations` | Inscriptions par periode | `?period=7days/30days/year` |
| GET | `/api/admin/stats/emotions-distribution` | Distribution emotions globale | `?period=30days` |
| GET | `/api/admin/activity` | Activite recente | `?limit=10` |

---

## Resume

| Module | Nb endpoints |
|--------|-------------|
| Auth | 5 |
| Users | 3 |
| Tracker | 6 |
| Reports | 3 |
| Emotions | 2 |
| Information | 2 |
| Admin Users | 6 |
| Admin Contents | 5 |
| Admin Emotions | 7 |
| Admin Dashboard | 4 |
| **Total** | **43** |

---

## Notes techniques

### Authentification
- Utiliser JWT pour les tokens (expiration 24h)
- Les endpoints `/api/admin/*` necessitent le role `administrateur`
- Les endpoints `/api/tracker/*` et `/api/reports/*` necessitent une authentification utilisateur
- Les endpoints `/api/emotions` et `/api/information/*` sont publics

### Rate limiting
- Les endpoints `/api/auth/*` sont soumis a un rate limiting : **2 requetes/seconde**, burst de 5
- Reponse `429 Too Many Requests` si la limite est depassee

### Validation des donnees (serveur)
- **Email** : doit contenir `@` et `.`, minimum 5 caracteres
- **Mot de passe** : minimum 8 caracteres, au moins 1 majuscule, 1 minuscule, 1 chiffre
- **Intensite tracker** : entre 1 et 10 (valide en creation ET en modification)

### Format des reponses
```json
{
  "success": true,
  "data": { ... },
  "message": "Optional message"
}
```

### Format des erreurs
```json
{
  "success": false,
  "error": {
    "code": "ERROR_CODE",
    "message": "Description de l'erreur"
  }
}
```

### Pagination
```json
{
  "data": [...],
  "pagination": {
    "page": 1,
    "limit": 10,
    "total": 100,
    "total_pages": 10
  }
}
```
