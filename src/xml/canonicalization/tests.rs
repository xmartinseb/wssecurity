#[cfg(test)]
mod tests {
    use crate::xml::canonicalization::CanonizedXml;

    fn test_xml(valid_xml_input: &str, expected_output: &str) {
        let canoni = CanonizedXml::new(valid_xml_input).unwrap();
        assert_eq!(canoni.as_str(), expected_output)
    }

    #[test]
    fn test_default_xmlns() {
        test_xml(
            r#"
        <A xmlns="http://a.a" xmlns:f="http://f.f">
            <B xmlns="http://a.a">
                <C xmlns="http://c.c">
                </C>
            </B>
        </A>
        "#,
            r#"<A xmlns="http://a.a"><B><C xmlns="http://c.c"></C></B></A>"#,
        );
    }

    #[test]
    fn test_xmlns_with_prefix() {
        test_xml(
            r#"
        <a:A xmlns:a="http://a.a">
            <b:B xmlns:b="http://a.a" xmlns:f="http://f.f">
                <c:C xmlns:c="http://c.c" xmlns:d="http://c.c" xmlns:e="http://e.e"  />
            </b:B>
        </a:A>
        "#,
            r#"<a:A xmlns:a="http://a.a"><b:B xmlns:b="http://a.a"><c:C xmlns:c="http://c.c"></c:C></b:B></a:A>"#,
        );
    }

    #[test]
    fn test_sort_attrs_one_unused_xmlns() {
        // Tady vyzkousim razeni atributu a vypadnuti nepouziteho namespace unused
        test_xml(
            r#"<A id="77" xmlns='http://def.ault' xmlns:z="http://z.z" d:id="a" z:id="q" xmlns:d="http://d.d" xmlns:unused="http://unu.sed" />"#,
            r#"<A xmlns="http://def.ault" xmlns:d="http://d.d" xmlns:z="http://z.z" id="77" d:id="a" z:id="q"></A>"#,
        );
    }

    #[test]
    fn test_unused_default_xmlns() {
        // Tady vyzkousim razeni atributu a vypadnuti nepouziteho namespace unused
        test_xml(
            r#"<z:A id="77" xmlns='http://def.ault' xmlns:z="http://z.z" d:id="a" z:id="q" xmlns:d="http://d.d"></z:A>"#,
            r#"<z:A xmlns:d="http://d.d" xmlns:z="http://z.z" id="77" d:id="a" z:id="q"></z:A>"#,
        );
    }

    #[test]
    fn test_default_xmlns_moves_to_child() {
        // Tady vyzkousim razeni atributu a vypadnuti nepouziteho namespace unused
        test_xml(
            r#"<z:A id="77" xmlns='http://def.ault' xmlns:z="http://z.z" d:id="a" z:id="q" xmlns:d="http://d.d"><B /></z:A>"#,
            r#"<z:A xmlns:d="http://d.d" xmlns:z="http://z.z" id="77" d:id="a" z:id="q"><B xmlns="http://def.ault"></B></z:A>"#,
        );
    }

    #[test]
    fn test_prefix_xmlns_moves_to_child() {
        // Tady vyzkousim razeni atributu a vypadnuti nepouziteho namespace unused
        test_xml(
            r#"
            <A xmlns:da="http://d.a">
                <da:B />
                <da:C />
                <da:D xmlns:da="http://D.D" />
            </A>"#,
            r#"<A><da:B xmlns:da="http://d.a"></da:B><da:C xmlns:da="http://d.a"></da:C><da:D xmlns:da="http://D.D"></da:D></A>"#,
        );
    }

    #[test]
    fn test_complicated_example() {
        // Tady vyzkousim razeni atributu a vypadnuti nepouziteho namespace unused
        test_xml(
            r#"
            <w:world xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"  
       xmlns:w="http://w.w"
       xmlns:extra="http://example.com/extra"
       xmlns="http://default.namespace"
       extra:note="Some extrainfo">

    <!-- Toto je komentář o státech světa -->

    <country capital="Tokyo" name="Japan" 
             xmlns:unused="http://useless.namespace.com"
             population="125.8 million" continent="Asia">
        <currency>Yen</currency>     
        <language primary="true" script="kanji kana">Japanese</language>
        <area>377975</area> 
        <emptyTag      />  

    </country>

    <country name="Brazil" continent="South America" 
             capital="Brasília"  
             xmlns:extra2="http://another-useless.namespace"
             population="213 million">

        <currency>Real</currency>
        <language primary="true">Portuguese</language> 
           <area>8515767</area>
    </country>  

    <country xmlns="http://eu.eu" continent="Europe"  population="83 million" name="Germany"   
             capital="Berlin" 
             xmlns:why="http://why.not/use/more/ns">
        <currency>Euro</currency>     
        <language primary="true"  note="used widely">German</language>
        <area>357386</area>
    </country>

</w:world>

"#,
            r#"<w:world xmlns:extra="http://example.com/extra" xmlns:w="http://w.w" extra:note="Some extrainfo"><country xmlns="http://default.namespace" capital="Tokyo" continent="Asia" name="Japan" population="125.8 million"><currency>Yen</currency><language primary="true" script="kanji kana">Japanese</language><area>377975</area><emptyTag></emptyTag></country><country xmlns="http://default.namespace" capital="Brasília" continent="South America" name="Brazil" population="213 million"><currency>Real</currency><language primary="true">Portuguese</language><area>8515767</area></country><country xmlns="http://eu.eu" capital="Berlin" continent="Europe" name="Germany" population="83 million"><currency>Euro</currency><language note="used widely" primary="true">German</language><area>357386</area></country></w:world>"#,
        );
    }
}
