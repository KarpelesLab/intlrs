#![cfg(feature = "segmentation-dict-cjk")]
//! Dictionary-based Chinese/Japanese word segmentation (feature
//! `segmentation-dict-cjk`).
//!
//! The expected segmentations below were produced by V8's
//! `Intl.Segmenter('zh' | 'ja', {granularity:'word'})`, which uses the same ICU
//! `cjdict` + `CjkBreakEngine` Viterbi this module ports. Every case here — real
//! Chinese and Japanese sentences plus mixed-script/punctuation text — matches
//! V8 exactly, tokens and all (V8 emits whitespace/punctuation as their own
//! segments, and so does [`words`]).
//!
//! Like ICU/V8, this port NFKC-normalizes the run before dictionary lookup, so
//! half-width katakana (U+FF66..=U+FF9F), full-width forms and the like segment
//! as their canonical equivalents while the emitted tokens keep their original
//! spelling (see [`nfkc_pre_normalization_matches_v8`]). Normal NFC/NFKC-stable
//! CJK — the overwhelmingly common case — is untouched (identity mapping).

use intl::unicode::words;

fn w(s: &str) -> Vec<&str> {
    words(s).collect()
}

#[test]
fn chinese_sentences_match_v8() {
    let cases: &[(&str, &[&str])] = &[
        ("我们都是中国人", &["我们", "都是", "中国人"]),
        (
            "今天天气很好我们去公园散步吧",
            &["今天", "天气", "很好", "我们", "去", "公园", "散步", "吧"],
        ),
        (
            "中华人民共和国成立于一九四九年",
            &["中华", "人民", "共和国", "成立", "于", "一九四九年"],
        ),
        (
            "他在北京大学学习计算机科学",
            &["他在", "北京", "大学", "学习", "计算", "机", "科学"],
        ),
        ("请问你叫什么名字", &["请问", "你", "叫", "什么", "名字"]),
        (
            "我喜欢吃苹果和香蕉",
            &["我", "喜欢", "吃", "苹果", "和", "香蕉"],
        ),
        (
            "上海是中国最大的城市之一",
            &["上海", "是", "中国", "最大", "的", "城市", "之一"],
        ),
        (
            "人工智能正在改变世界",
            &["人工", "智能", "正在", "改变", "世界"],
        ),
        (
            "机器学习和深度学习是人工智能的重要分支",
            &[
                "机器", "学习", "和", "深度", "学习", "是", "人工", "智能", "的", "重要", "分支",
            ],
        ),
        ("广州深圳香港澳门", &["广州", "深圳", "香港", "澳门"]),
        (
            "北京2022年冬季奥林匹克运动会",
            &["北京", "2022", "年", "冬季", "奥林匹克", "运动", "会"],
        ),
        (
            "习近平主席发表重要讲话",
            &["习", "近平", "主席", "发表", "重要", "讲话"],
        ),
    ];
    for (input, expected) in cases {
        assert_eq!(&w(input), expected, "input: {input}");
    }
}

#[test]
fn japanese_sentences_match_v8() {
    let cases: &[(&str, &[&str])] = &[
        (
            "私は日本語を勉強しています",
            &["私", "は", "日本語", "を", "勉強", "し", "てい", "ます"],
        ),
        (
            "今日はいい天気ですね",
            &["今日", "は", "いい", "天気", "です", "ね"],
        ),
        (
            "東京は日本の首都です",
            &["東京", "は", "日本", "の", "首都", "です"],
        ),
        (
            "彼女はコンピュータープログラマーです",
            &["彼女", "は", "コンピューター", "プログラマー", "です"],
        ),
        (
            "寿司と天ぷらが大好きです",
            &["寿司", "と", "天ぷら", "が", "大好き", "です"],
        ),
        (
            "来週の月曜日に会議があります",
            &["来週", "の", "月曜日", "に", "会議", "が", "あり", "ます"],
        ),
        (
            "京都には古いお寺がたくさんあります",
            &[
                "京都",
                "に",
                "は",
                "古い",
                "お寺",
                "が",
                "たくさん",
                "あり",
                "ます",
            ],
        ),
        (
            "吾輩は猫である名前はまだ無い",
            &[
                "吾輩", "は", "猫", "で", "ある", "名前", "は", "まだ", "無い",
            ],
        ),
        (
            "データベースにアクセスする",
            &["データベース", "に", "アクセス", "する"],
        ),
        (
            "オリンピックとパラリンピックが開催される",
            &[
                "オリンピック",
                "と",
                "パラリンピック",
                "が",
                "開催",
                "さ",
                "れる",
            ],
        ),
    ];
    for (input, expected) in cases {
        assert_eq!(&w(input), expected, "input: {input}");
    }
}

#[test]
fn mixed_script_and_punctuation_match_v8() {
    // The CJK run is subdivided by the dictionary; surrounding punctuation,
    // Latin, and digit tokens keep their exact UAX #29 behavior — exactly as V8.
    let cases: &[(&str, &[&str])] = &[
        (
            "「你好，世界！」他说。",
            &["「", "你好", "，", "世界", "！", "」", "他", "说", "。"],
        ),
        (
            "我有3个苹果和5个橘子。",
            &["我有", "3", "个", "苹果", "和", "5", "个", "橘子", "。"],
        ),
        (
            "中文English混合text测试",
            &["中文", "English", "混合", "text", "测试"],
        ),
        (
            "他说：「明天见！」然后离开了。",
            &[
                "他", "说", "：", "「", "明天", "见", "！", "」", "然后", "离开", "了", "。",
            ],
        ),
        (
            "すみません、駅はどこですか？",
            &["すみません", "、", "駅", "は", "どこ", "です", "か", "？"],
        ),
    ];
    for (input, expected) in cases {
        assert_eq!(&w(input), expected, "input: {input}");
    }
}

#[test]
fn thai_still_works_alongside_cjk() {
    // The Thai engine (feature `segmentation-dict`) is unaffected by adding CJK.
    assert_eq!(
        w("ไปกินข้าวที่ร้านอาหารกับเพื่อน"),
        ["ไป", "กิน", "ข้าว", "ที่", "ร้าน", "อาหาร", "กับ", "เพื่อน"]
    );
}

#[test]
fn non_cjk_scripts_unaffected() {
    // Hangul is deliberately NOT routed to the CJK engine (Korean uses spaces).
    assert_eq!(w("안녕하세요 세계"), ["안녕하세요", " ", "세계"]);
    assert_eq!(w("Hello, world!"), ["Hello", ",", " ", "world", "!"]);
}

#[test]
fn nfkc_pre_normalization_matches_v8() {
    // ICU/V8 NFKC-normalize the run before dictionary lookup, then map the
    // breaks back onto the ORIGINAL text — so half-width katakana and full-width
    // forms segment like their canonical equivalents but the tokens keep their
    // original spelling. Every expected value below is V8's
    // `Intl.Segmenter('ja',{granularity:'word'})` output.
    let cases: &[(&str, &[&str])] = &[
        // Half-width katakana, including a voiced sound mark that composes across
        // the code-point join (ﾃﾞ → デ): the break falls before ﾃ, never inside it.
        ("ﾃｽﾄﾃﾞｰﾀ", &["ﾃｽﾄ", "ﾃﾞｰﾀ"]),
        ("ﾃｽﾄ", &["ﾃｽﾄ"]),
        ("ｺﾝﾋﾟｭｰﾀｰ", &["ｺﾝﾋﾟｭｰﾀｰ"]),
        ("ﾊﾟﾋﾟﾌﾟﾍﾟﾎﾟ", &["ﾊﾟﾋﾟﾌﾟﾍﾟﾎﾟ"]),
        ("ｼﾞｬﾊﾟﾝ", &["ｼﾞｬﾊﾟﾝ"]),
        // Han + half-width katakana in one dictionary run.
        ("東京ﾀﾜｰ", &["東京ﾀﾜｰ"]),
        ("半角ｶﾀｶﾅ", &["半角", "ｶﾀｶﾅ"]),
        // Full-width Latin / digits are handled by UAX #29 (not the CJK run), but
        // must still line up exactly with V8 at the script boundary.
        ("ＡＢＣ東京", &["ＡＢＣ", "東京"]),
        ("１２３東京", &["１２３", "東京"]),
        // No-regression: already-NFC text is an identity mapping.
        ("東京タワー", &["東京タワー"]),
        ("テストデータ", &["テスト", "データ"]),
    ];
    for (input, expected) in cases {
        assert_eq!(&w(input), expected, "input: {input}");
    }
}

#[test]
fn short_cjk_runs() {
    // A single ideograph is one word; a mixed sentence splits at script runs.
    assert_eq!(w("人"), ["人"]);
    assert_eq!(w("中国"), ["中国"]);
}
