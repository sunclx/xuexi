-- SQLite
SELECT id, category, content, options, answer, notes, bounds
FROM `banks`
WHERE answer="";

DELETE
FROM banks
WHERE answer="";