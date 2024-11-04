-- 教材版本表
CREATE TABLE textbook_versions (
    id SERIAL PRIMARY KEY,
    name VARCHAR(50) NOT NULL,     -- 例如：人教版、苏教版
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- 年级表
CREATE TABLE grades (
    id SERIAL PRIMARY KEY,
    name VARCHAR(50) NOT NULL,     -- 例如：一年级、二年级
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- 学期表 (固定上下学期)
CREATE TABLE semesters (
    id SERIAL PRIMARY KEY,
    name VARCHAR(50) NOT NULL,     -- 上学期、下学期
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- 教材表 (关联版本和年级)
CREATE TABLE textbooks (
    id SERIAL PRIMARY KEY,
    version_id INTEGER NOT NULL,
    grade_id INTEGER NOT NULL,
    semester_id INTEGER NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(version_id, grade_id, semester_id)
);

-- 单元表
CREATE TABLE units (
    id SERIAL PRIMARY KEY,
    name VARCHAR(50) NOT NULL,
    textbook_id INTEGER NOT NULL,
    sequence_number INTEGER NOT NULL,  -- 单元序号
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- 词汇单元关联表 (记录单词在哪些单元中出现)
CREATE TABLE word_unit_mapping (
    id SERIAL PRIMARY KEY,
    word_id INTEGER NOT NULL,      -- 关联到 words 表的 word_id
    unit_id INTEGER NOT NULL,
    meaning TEXT NOT NULL,         -- 在该单元中的具体含义/翻译
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(word_id, unit_id)       -- 确保同一个单词在同一单元不会重复
);

-- 创建索引
CREATE INDEX idx_textbooks_version_id ON textbooks(version_id);
CREATE INDEX idx_textbooks_grade_id ON textbooks(grade_id);
CREATE INDEX idx_textbooks_semester_id ON textbooks(semester_id);
CREATE INDEX idx_units_textbook_id ON units(textbook_id);
CREATE INDEX idx_word_unit_mapping_word_id ON word_unit_mapping(word_id);
CREATE INDEX idx_word_unit_mapping_unit_id ON word_unit_mapping(unit_id);

-- 插入基础数据
INSERT INTO textbook_versions (name) VALUES 
    ('人教版'),
    ('苏教版');

INSERT INTO grades (name) VALUES 
    ('一年级'),
    ('二年级'),
    ('三年级'),
    ('四年级'),
    ('五年级'),
    ('六年级'),
    ('七年级'),
    ('八年级'),
    ('九年级'),
    ('高一'),
    ('高二'),
    ('高三');


INSERT INTO semesters (name) VALUES 
    ('上学期'),
    ('下学期');