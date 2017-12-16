import React from 'react'
import {
    BrowserRouter as Router,
    Route,
    Link,
    Redirect
} from 'react-router-dom'
import {
    Card,
    CardBody,
    CardHeader
} from 'reactstrap';
import marked from 'marked';
import highlightjs from 'highlight.js';

class Page extends React.Component {
    constructor(props) {
        super(props);
        this.state = {
            json: [],
            404: false
        };
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

    componentDidMount() {
        var path = window.location.pathname;
        if(path[path.length - 1] === '/') {
            path = path.substring(0, path.length - 1);
        }
        return fetch(path + '.json', {method: 'GET'})
            .then(response => {
                if(response.ok) {
                    return response.json();
                } else {
                    this.state[404] = true;
                    return []
                }
            })
            .then(json => {
                this.setState({
                    json: json
                });
            });
    }

    render() {
        if(this.state[404]) {
            return <Redirect push to='/404' />
        }

        return (
            <div className='project'>
                {this.state.json.map(function(card, i) {
                    return (
                        <Card key={i}>
                            <CardHeader>
                                <center>
                                    <h3>{card.title}</h3>
                                </center>
                            </CardHeader>
                            <CardBody className='md' dangerouslySetInnerHTML={{__html: marked(card.text.join('\n'))}} />
                        </Card>
                    );
                })}
            </div>
                );
    }

    };

    export default Page;
