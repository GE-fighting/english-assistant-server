-- 创建数据库
CREATE DATABASE english_assitant;


-- 创建words表
CREATE TABLE words (
    word_id SERIAL PRIMARY KEY,
    word VARCHAR(100) NOT NULL,
    phonetic VARCHAR(150),
    pronunciation_base64 TEXT,  -- Base64编码的音频文件
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
--words表添加唯一索引
CREATE UNIQUE INDEX word_unique_idx ON words (word);
