import React from 'react'
import {
    Redirect,
    Link
} from 'react-router-dom'
import Markdown from './Markdown.jsx';

class Page extends React.Component {
    constructor(props) {
        super(props);
        this.state = {
            json: [],
            404: false
        };

        if(props.type === 'frontpage') {
            fetch('/posts.json', {method: 'GET'})
                .then(response => response.json())
                .then(json => {
                    json = json.filter(function(item) {
                        return item.tags.includes('blog');
                    })[0];
                    fetch(json.link + '.json', {method: 'GET'})
                        .then(response => response.json())
                        .then(json => {
                            this.setState({
                                json: json
                            });
                        });
                });
        } else {
            var path = window.location.pathname;
            if(path[path.length - 1] === '/') {
                path = path.substring(0, path.length - 1);
            }
            fetch(path + '.json', {method: 'GET'})
                .then(response => {
                    if(response.ok) {
                        return response.json();
                    } else {
                        this.state[404] = true;
                        return [];
                    }
                })
                .then(json => {
                    this.setState({
                        json: json
                    });
                });
        }
    }

    componentDidUpdate() {
        MathJax.Hub.Queue(["Typeset",MathJax.Hub]);
    }


    render() {
        if(this.state[404]) {
            return <Redirect push to='/404' />
        }

        function Title(props) {
            if(props.date) {
                return (
                    <div className='card-header'>
                        <h2 className='card-title'>{props.title}</h2>
                        <h5 className='card-date'>{props.date}</h5>
                    </div>
                );
            } else {
                return (
                    <div className='card-header'>
                        <center>
                            <h2>{props.title}</h2>
                        </center>
                    </div>
                );
            }
        }

        function Tags(props) {
            if(props.tags && props.last) {
                return (
                    <div className='card-tags'>
                        {props.tags.map(function(tag, i) {
                            var tag_element = [<Link key={2 * i} className='card-tag' to={'/tag/' + tag}>{tag}</Link>];
                            if(i < props.tags.length - 1) {
                                tag_element.push(<span key={2 * i + 1} className='comma'>, </span>);
                            }
                            return tag_element;
                        })}
                    </div>
                );
            }
            return '';
        }

        return (
            <div className='page'>
                {this.state.json.map(function(card, i) {
                    return (
                        <div className='card' key={i}>
                            <Title title={card.title} date={card.date} />
                            <Markdown className='card-body' markdown={card.text.join('\n')} />
                            <Tags tags={card.tags} last={i === this.state.json.length - 1} />
                        </div>
                    );
                }, this)}
            </div>
        );
    }

};

export default Page;
