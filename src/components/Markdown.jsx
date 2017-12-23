import React from 'react'
import marked from 'marked';
import highlightjs from 'highlight.js';

class Markdown extends React.Component {
    constructor(props) {
        super(props);
        /* https://shuheikagawa.com/blog/2015/09/21/using-highlight-js-with-marked/ */
        const renderer = new marked.Renderer();
        renderer.code = (code, language) => {
            // Check whether the given language is valid for highlight.js.
            const validLang = !!(language && highlightjs.getLanguage(language));
            // Highlight only if the language is valid.
            const highlighted = validLang ? highlightjs.highlight(language, code).value : code;
            // Render the highlighted code with `hljs` class.
            return `<pre><code class="hljs ${language}">${highlighted}</code></pre>`;
        };
        marked.setOptions({
            renderer: renderer,
            gfm: true,
            tables: true,
            breaks: false,
            pedantic: false,
            sanitize: false,
            smartLists: true,
            smartypants: false,
        });
    }

    render() {
        return (
            <div className={'md ' + this.props.className} dangerouslySetInnerHTML={{__html: marked(this.props.markdown)}} />
        );
    }

};

export default Markdown;
