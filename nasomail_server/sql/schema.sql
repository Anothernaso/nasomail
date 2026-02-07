PRAGMA foreign_keys = ON;

CREATE TABLE IF NOT EXISTS users (
    id         INTEGER  PRIMARY KEY,

    name TEXT UNIQUE NOT NULL COLLATE NOCASE
        CHECK (name = TRIM(name) AND LENGTH(name) >=  3 AND LENGTH(name) <=  20),

    passphrase TEXT NOT NULL
        CHECK (passphrase = TRIM(passphrase) AND LENGTH(passphrase) >=  8 AND LENGTH(passphrase) <=  20),

    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS mails (
    id         INTEGER  PRIMARY KEY,
    user_id    INTEGER  NOT NULL,

    subject    TEXT     NOT NULL
        CHECK (subject = TRIM(subject) AND LENGTH(subject) <=  255),

    body       TEXT     NOT NULL
        CHECK (body = TRIM(body) AND LENGTH(body) <=  1024*1024),

    sender     TEXT     NOT NULL
        CHECK (sender = TRIM(sender) AND LENGTH(sender) <=  255),

    recipient  TEXT     NOT NULL
        CHECK (recipient = TRIM(recipient) AND LENGTH(recipient) <=  255),

    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,

    status     TEXT     NOT NULL
        CHECK (status IN ('new', 'read', 'draft', 'sent')),

    FOREIGN KEY (user_id)
        REFERENCES users(id)
        ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS attachments (
    id         INTEGER  PRIMARY KEY,
    mail_id    INTEGER  NOT NULL,

    data       BLOB     NOT NULL
        CHECK (length(data) <= 1024*1024*1000),

    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,

    FOREIGN KEY (mail_id)
        REFERENCES mails(id)
        ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_mails_user_id
    ON mails(user_id);

CREATE INDEX IF NOT EXISTS idx_attachments_mail_id
    ON attachments(mail_id);
