-- Role
CREATE TABLE role (
    id SERIAL PRIMARY KEY,
    nom VARCHAR(50) NOT NULL UNIQUE,
    description VARCHAR(255)
);

-- Utilisateur
CREATE TABLE utilisateur (
    id SERIAL PRIMARY KEY,
    email VARCHAR(255) NOT NULL UNIQUE,
    mot_de_passe_hash VARCHAR(255) NOT NULL,
    nom VARCHAR(100) NOT NULL,
    prenom VARCHAR(100) NOT NULL,
    date_naissance DATE,
    est_actif BOOLEAN NOT NULL DEFAULT TRUE,
    role_id INTEGER NOT NULL DEFAULT 1 REFERENCES role(id),
    date_creation TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    date_modification TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX idx_utilisateur_email ON utilisateur(email);

-- Catégorie de contenu
CREATE TABLE categorie_contenu (
    id SERIAL PRIMARY KEY,
    nom VARCHAR(100) NOT NULL UNIQUE,
    description TEXT,
    ordre_affichage INTEGER NOT NULL DEFAULT 0,
    est_active BOOLEAN NOT NULL DEFAULT TRUE
);

-- Page d'information
CREATE TABLE page_information (
    id SERIAL PRIMARY KEY,
    titre VARCHAR(255) NOT NULL,
    contenu TEXT NOT NULL,
    resume VARCHAR(500),
    categorie_id INTEGER REFERENCES categorie_contenu(id) ON DELETE SET NULL,
    est_publiee BOOLEAN NOT NULL DEFAULT FALSE,
    auteur_id INTEGER NOT NULL REFERENCES utilisateur(id),
    date_creation TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    date_modification TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX idx_page_categorie ON page_information(categorie_id);

-- Émotion de base
CREATE TABLE emotion_base (
    id SERIAL PRIMARY KEY,
    nom VARCHAR(50) NOT NULL UNIQUE,
    emoji VARCHAR(10),
    couleur VARCHAR(7)
);

-- Sous-émotion
CREATE TABLE sous_emotion (
    id SERIAL PRIMARY KEY,
    nom VARCHAR(50) NOT NULL,
    emotion_base_id INTEGER NOT NULL REFERENCES emotion_base(id),
    UNIQUE(nom, emotion_base_id)
);

-- Entrée du tracker
CREATE TABLE entree_tracker (
    id SERIAL PRIMARY KEY,
    utilisateur_id INTEGER NOT NULL REFERENCES utilisateur(id),
    sous_emotion_id INTEGER NOT NULL REFERENCES sous_emotion(id),
    intensite INTEGER NOT NULL CHECK(intensite >= 1 AND intensite <= 10),
    note TEXT,
    date_entree TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    date_modification TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX idx_entree_utilisateur ON entree_tracker(utilisateur_id);
CREATE INDEX idx_entree_date ON entree_tracker(date_entree);
CREATE INDEX idx_entree_utilisateur_date ON entree_tracker(utilisateur_id, date_entree);
