#[rstest::rstest]
#[case("no math here", ("no math here".to_string(), vec![]))]
#[case("$$x^2$$", ("__LATEX_0__".to_string(), vec!["$$x^2$$".to_string()]))]
#[case("$x + y$", ("__LATEX_0__".to_string(), vec!["$x + y$".to_string()]))]
#[case("text $$a^2$$ more", ("text __LATEX_0__ more".to_string(), vec!["$$a^2$$".to_string()]))]
#[case("before $x$ after", ("before __LATEX_0__ after".to_string(), vec!["$x$".to_string()]))]
#[case("$$first$$ $$second$$", ("__LATEX_0__ __LATEX_1__".to_string(), vec!["$$first$$".to_string(), "$$second$$".to_string()]))]
#[case("$a$ and $b$", ("__LATEX_0__ and __LATEX_1__".to_string(), vec!["$a$".to_string(), "$b$".to_string()]))]
#[case("$$display$$ then $inline$", ("__LATEX_0__ then __LATEX_1__".to_string(), vec!["$$display$$".to_string(), "$inline$".to_string()]))]
#[case("$$x^2<br>y$$", ("__LATEX_0__".to_string(), vec!["$$x^2<br>y$$".to_string()]))]
#[case("$a<br>b$", ("__LATEX_0__".to_string(), vec!["$a<br>b$".to_string()]))]
#[case("$$multi\nline$$", ("__LATEX_0__".to_string(), vec!["$$multi\nline$$".to_string()]))]
#[case("text $$eq1$$ middle $var$ end", ("text __LATEX_0__ middle __LATEX_1__ end".to_string(), vec!["$$eq1$$".to_string(), "$var$".to_string()]))]
#[case("$x$ $$y^2$$ $z$", ("__LATEX_1__ __LATEX_0__ __LATEX_2__".to_string(), vec!["$$y^2$$".to_string(), "$x$".to_string(), "$z$".to_string()]))]
#[case("$$a<br/>b<br>c$$", ("__LATEX_0__".to_string(), vec!["$$a<br/>b<br>c$$".to_string()]))]
#[case("prefix$no_space$suffix", ("prefix__LATEX_0__suffix".to_string(), vec!["$no_space$".to_string()]))]
#[case("$$escaped\\$dollar$$", ("__LATEX_0__".to_string(), vec!["$$escaped\\$dollar$$".to_string()]))]
#[case("$\\$escaped$", ("__LATEX_0__".to_string(), vec!["$\\$escaped$".to_string()]))]
fn protect_latex(#[case] input: &str, #[case] expected: (String, Vec<String>)) {
    let output = web_scraper::examside::content_cleaner::protect_latex(input);
    assert_eq!(output, expected);
}

#[rstest::rstest]
#[case("$$x^2$$", "$$x^2$$")]
#[case("$a + b$", "$a + b$")]
#[case("not_math", "not_math")]
#[case("$$x^2<br>y^2$$", "$$x^2\\newline y^2$$")]
#[case("$$a<br/>b$$", "$$a\\newline b$$")]
#[case("$$first<br />second$$", "$$first\\newline second$$")]
#[case("$$x<BR>y$$", "$$x\\newline y$$")]
#[case("$$start<BR/>end$$", "$$start\\newline end$$")]
#[case(
    "$$line1<br>line2<br/>line3$$",
    "$$line1\\newline line2\\newline line3$$"
)]
#[case("$a<br>b$", "$ab$")]
#[case("$x<br/>y$", "$xy$")]
#[case("$var<BR />name$", "$varname$")]
#[case("$p<br>q<br/>r$", "$pqr$")]
#[case(
    "$$display with spaces<br>and content$$",
    "$$display with spaces\\newline and content$$"
)]
#[case("$inline<BR>math$", "$inlinemath$")]
#[case("$$<br>$$", "$$\\newline $$")]
#[case("$<br/>$", "$$")]
#[case("$$x^2 + y^2$$", "$$x^2 + y^2$$")]
#[case("$a = b$", "$a = b$")]
#[case(
    "$$multiple<br>lines<br>here$$",
    "$$multiple\\newline lines\\newline here$$"
)]
fn sanitize_latex_token(#[case] input: &str, #[case] expected: &str) {
    let output = web_scraper::examside::content_cleaner::sanitize_latex_token(input);
    assert_eq!(output, expected);
}

#[rstest::rstest]
#[case("no placeholders", &[], "no placeholders")]
#[case("__LATEX_0__", &["$$x^2$$".to_string()], "$$x^2$$")]
#[case("text __LATEX_0__ more", &["$$a + b$$".to_string()], "text $$a + b$$ more")]
#[case("__LATEX_0__ __LATEX_1__", &["$$first$$".to_string(), "$second$".to_string()], "$$first$$ $second$")]
#[case("__LATEX_0__ and __LATEX_1__", &["$x$".to_string(), "$y$".to_string()], "$x$ and $y$")]
#[case("before __LATEX_0__ middle __LATEX_1__ after", &["$$eq1$$".to_string(), "$var$".to_string()], "before $$eq1$$ middle $var$ after")]
#[case("__LATEX_0__ __LATEX_1__ __LATEX_2__", &["$a$".to_string(), "$$b$$".to_string(), "$c$".to_string()], "$a$ $$b$$ $c$")]
#[case("__LATEX_0__ text __LATEX_0__", &["$$x$$".to_string()], "$$x$$ text $$x$$")]
#[case("start __LATEX_0__ end", &["$$x^2\\newliney^2$$".to_string()], "start $$x^2\\newliney^2$$ end")]
#[case("__LATEX_0__", &["$a + b$".to_string()], "$a + b$")]
#[case("multiple __LATEX_0__ and __LATEX_1__ and __LATEX_0__", &["$$first$$".to_string(), "$second$".to_string()], "multiple $$first$$ and $second$ and $$first$$")]
#[case("text", &[], "text")]
#[case("__LATEX_0__", &[], "")]
#[case("__LATEX_5__", &["$a$".to_string()], "")]
#[case("__LATEX_0____LATEX_1__", &["$$a$$".to_string(), "$$b$$".to_string()], "$$a$$$$b$$")]
#[case("prefix__LATEX_0__suffix", &["middle".to_string()], "prefixmiddlesuffix")]
#[case("__LATEX_0__ $$x^2<br>y$$ __LATEX_1__", &["$$x^2\\newliney$$".to_string(), "$a + b$".to_string()], "$$x^2\\newliney$$ $$x^2<br>y$$ $a + b$")]
#[case("equation: __LATEX_0__ and formula: __LATEX_1__", &["$$E=mc^2$$".to_string(), "$F=ma$".to_string()], "equation: $$E=mc^2$$ and formula: $F=ma$")]
#[case("__LATEX_0____LATEX_0____LATEX_0__", &["x".to_string()], "xxx")]
#[case("no match here __LATEX_", &["$$test$$".to_string()], "no match here __LATEX_")]
fn restore_latex(#[case] input: &str, #[case] tokens: &[String], #[case] expected: &str) {
    let output = web_scraper::examside::content_cleaner::restore_latex(input, tokens);
    assert_eq!(output, expected);
}

#[tokio::test]
#[rstest::rstest]
#[case("<p>Hello world</p>", "<p>Hello world</p>")]
#[case("$$x^2 + y^2$$", "$$x^2 + y^2$$")]
#[case("$a + b = c$", "$a + b = c$")]
#[case("$$x^2<br>+ y^2$$", "$$x^2\\newline + y^2$$")]
#[case("$$a<br/>b$$", "$$a\\newline b$$")]
#[case("$$first<br />second$$", "$$first\\newline second$$")]
#[case("$$x<BR>y$$", "$$x\\newline y$$")]
#[case("$$start<BR/>end$$", "$$start\\newline end$$")]
#[case(
    "$$line1<br>line2<br/>line3<br />line4$$",
    "$$line1\\newline line2\\newline line3\\newline line4$$"
)]
#[case("$a<br>+ b$", "$a+ b$")]
#[case("$x<br/>y$", "$xy$")]
#[case("$var<BR />name$", "$varname$")]
#[case("$a<br>b<br/>c$", "$abc$")]
#[case("$$eq1$$ and $$eq2$$", "$$eq1$$ and $$eq2$$")]
#[case("$a$ and $b$", "$a$ and $b$")]
#[case("$$display$$ then $inline$", "$$display$$ then $inline$")]
#[case("<p>Line 1<br>Line 2</p>", "<p>Line 1<br>Line 2</p>")]
#[case("<p>A<br/>B</p>", "<p>A<br>B</p>")]
#[case(
    "<p>Text <br> $$x^2<br>y$$ more</p>",
    "<p>Text <br> $$x^2\\newline y$$ more</p>"
)]
#[case("$$x^2\n+ y^2$$", "$$x^2\n+ y^2$$")]
#[case("$$<span>x</span>^2$$", "$$<span>x</span>^2$$")]
#[case("$$escaped\\$dollar$$", "$$escaped\\$dollar$$")]
#[case("$\\$escaped$", "$\\$escaped$")]
#[case("$$\\alpha + \\beta = \\gamma$$", "$$\\alpha + \\beta = \\gamma$$")]
#[case("$\\sin(x) + \\cos(x)$", "$\\sin(x) + \\cos(x)$")]
#[case("$$equation$$ text", "$$equation$$ text")]
#[case("text $$equation$$", "text $$equation$$")]
#[case("$$x^2$$", "$$x^2$$")]
#[case("", "")]
#[case("   \n\t  ", "   \n\t  ")]
#[case("$$$$", "$$$$")]
#[case("$$a$$$$b$$", "$$a$$$$b$$")]
#[case(
    "<p>some text <br> some text <table>...</table> $$display latex$$ $inline latex$ $$display latex with <br>$$ and </p>",
    "<p>some text <br> some text </p>...<table></table> $$display latex$$ $inline latex$ $$display latex with \\newline $$ and <p></p>"
)]
async fn clean(#[case] input: &str, #[case] expected: &str) {
    let output = web_scraper::examside::content_cleaner::clean(input)
        .await
        .unwrap();
    assert_eq!(output.as_str(), expected);
}
