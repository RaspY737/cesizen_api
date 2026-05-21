-- Documents de seed (les fichiers sont uploadés dans Minio via minio-init)
INSERT INTO document (nom, nom_fichier, taille, content_type, s3_key, uploaded_by) VALUES
    ('Guide de respiration - Cohérence cardiaque',
     'guide-respiration.md', 716, 'text/markdown',
     'public/seed/guide-respiration.md', 1),

    ('Échelle de Holmes et Rahe - Questionnaire',
     'echelle-holmes-rahe.md', 1924, 'text/markdown',
     'public/seed/echelle-holmes-rahe.md', 1),

    ('Charte du bien-être au quotidien',
     'charte-bien-etre.md', 1025, 'text/markdown',
     'public/seed/charte-bien-etre.md', 1)
ON CONFLICT (s3_key) DO NOTHING;
