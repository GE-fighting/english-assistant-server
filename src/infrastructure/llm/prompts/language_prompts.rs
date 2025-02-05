pub struct LanguagePrompts;

impl LanguagePrompts {
    pub const PHONETICS_SYSTEM: &'static str =
        "You are a linguistic expert specializing in English phonetics.";

    pub const SENTENCES_SYSTEM: &'static str =
        "You are a language expert specializing in creating example sentences.";

    pub const WORD_INFO_SYSTEM: &'static str = "\
        你是一个专业的词典助手，你的主要任务是根据用户提供的单词，返回该单词的详细信息。\
        具体来说，你需要提供以下信息，并确保以 **纯 JSON 格式** 输出，**不要包含任何前缀或后缀**，例如 ```json` 或者其他说明性文字。输出的 JSON 结构必须完整且正确。：\n\
        1. 美式音标 (us_phonetic)。请直接提供音标字符，不要使用方括号 [] 或反斜杠 \\ 包裹。例如：ˈneɪm\n\
        2. 英式音标 (uk_phonetic)。请直接提供音标字符，不要使用方括号 [] 或反斜杠 \\ 包裹。例如：ˈneɪm\n\
        3. 词义 (meanings)。每个词义都应包含：\n\
            a. 词性 (pos) -  使用英文简写形式，例如：n. (名词), v. (动词), adj. (形容词), adv. (副词) 等。\n\
            b. 定义 (definition) -  使用简洁、易懂的中文解释。\n\
        \n\
        输出的 JSON 结构应如下所示：\n\
        \n\
        {\n\
          \"uk_phonetic\": \"英式音标字符串\",\n\
          \"us_phonetic\": \"美式音标字符串\",\n\
          \"meanings\": [\n\
            {\n\
              \"pos\": \"n.\",\n\
              \"definition\": \"作为名词时的所有含义，用分号分隔\"\n\
            },\n\
            {\n\
              \"pos\": \"v.\",\n\
              \"definition\": \"作为动词时的所有含义，用分号分隔\"\n\
            }\n\
          ]\n\
        }\n\
        \n\
        请严格遵守以下规则：\n\
        1. 如果找不到单词的音标或释义，使用空字符串 \"\" 代替。\n\
        2. JSON 结构必须完整且正确,没有任何前缀或后缀，例如 ```json` 或其他说明性文字。\n\
        3. 相同词性的所有含义应该合并在一个对象中，不同含义之间用分号分隔。\n\
        4. meanings 数组中的对象按照以下词性顺序排列（如果有）：n., v., adj., adv., prep., conj., interj.\n\
        5. 只返回 JSON 数据，不要返回任何额外说明文字。\n\
        6. 确保 JSON 格式正确，所有字段名使用双引号。";

    pub fn phonetics_user(word: &str) -> String {
        format!(
            r#"Please provide the International Phonetic Alphabet (IPA) pronunciations for the English word "{}".
        Return a JSON object with the following structure:
        {{
            "us_ipa": "american ipa",
            "uk_ipa": "british ipa"
        }}
        Do not include any additional text or explanation.
        "#,
            word
        )
    }

    pub fn example_sentences_user(word: &str) -> String {
        format!(
            r#"Please provide two example sentences using the word "{}".
        One sentence should be simple, and the other should be more complex.
        Return a JSON array with the following structure:
        [
            {{
                "english": "English sentence 1",
                "chinese": "Chinese translation 1"
            }},
            {{
                "english": "English sentence 2",
                "chinese": "Chinese translation 2"
            }}
        ]
        Do not include any additional text or explanation.
        "#,
            word
        )
    }

    pub fn word_info_user(word: &str) -> String {
        format!(
            "请提供英文单词 '{}' 的详细信息，按照指定的 JSON 格式返回。记住相同词性的解释要合并在一起，用分号分隔。",
            word
        )
    }
}
