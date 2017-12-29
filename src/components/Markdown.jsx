import React from 'react';
import marked from 'marked';
import highlightjs from 'highlight.js';
import $ from 'jquery';

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
        $(document).ready(function() {
            $('a.spoiler').off('click').on('click',function() {
                $(this).next().toggleClass('active');
                $(this).toggleClass('active');
            });
        });
    }

    spoiler(text) {
        var start_tag = new RegExp('<spoiler ?([^>]*)>', 'g');
        var end_tag = new RegExp('</spoiler>', 'g');
        return text.replace(start_tag, function(match, p, offset, string) {
            if(p) {
                return '<a class="spoiler">' + p + '</a><span class="spoiler-content">'
            } else {
                return '<a class="spoiler">Spoiler</a><span class="spoiler-content">'
            }
        }).replace(end_tag,'</span>')
    }

    align(text) {
        var rstart_tag = new RegExp('<ralign>', 'g');
        var rend_tag = new RegExp('</ralign>', 'g');
        return text.replace(rstart_tag, '<div class="ralign">')
            .replace(rend_tag,'</div>');
    }

    render() {
        return (
            <div className={'md ' + this.props.className} dangerouslySetInnerHTML={{__html: marked(this.align(this.spoiler(this.props.markdown)))}} />
        );
    }

};

export default Markdown;
