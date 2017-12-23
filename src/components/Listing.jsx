import React from 'react'
import {
    BrowserRouter as Router,
    Route,
    Link,
    Redirect
} from 'react-router-dom'
import Markdown from './Markdown.jsx';
import $ from 'jquery';

class Listing extends React.Component {
    constructor(props) {
        super(props);
        this.state = {
            json: [],
            jsons: 0
        };

        if('type' in this.props) {
            fetch('/' + this.props.type + '.json', {method: 'GET'})
                .then(response => response.json())
                .then(json => {
                    this.setState({
                        json: json
                    });
                });
        } else {
            var tags = window.location.pathname.split('/tag/')
            if(tags.length > 1) {
                var tag = tags[1];
                var jsons = ['/posts.json', '/notes.json', '/chronocides.json', '/projects.json']
                for(let i = 0; i < jsons.length; i++) {
                    fetch(jsons[i], {method: 'GET'})
                        .then(response => response.json())
                        .then(json => {
                            json = json.filter(post => {
                                return post.tags.includes(tag);
                            });
                            Array.prototype.push.apply(this.state.json, json);
                            this.state.jsons += 1;
                            if(this.state.jsons == jsons.length) {
                                this.setState(this.state);
                            }
                        });
                }
            }
        }
    }

    render() {

        function Github(props) {
            if(props.github) {
                return (
                    <div className='github'>
                        <Link to={props.github}>
                            <i className="fa fa-github" aria-hidden="true"></i>
                        </Link>
                    </div>
                );
            }
            return <div className='github'></div>
        }

        function ReadMore(props) {
            return (
                <div className='read-more'>
                    <Link to={props.link}>
                        Read More
                    </Link>
                </div>
            );
        }

        function Tags(props) {
            if(props.tags) {
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

        function Card(props) {
            return (
                <div className='card'>
                    <div className='card-header'>
                        <center>
                            <h4>
                                <Link className='page-link' to={props.link}>
                                    {props.title}
                                </Link>
                            </h4>
                        </center>
                        <Github github={props.github} />
                    </div>
                    <Markdown className='card-body' markdown={props.text.join('\n')}/>
                    <div className='row'>
                        <Tags tags={props.tags} />
                        <ReadMore link={props.link} />
                    </div>
                </div>
            );
        }

        var json = this.state.json;
        if(json.length === 0 && window.location.pathname.startsWith('/tag/')) {
            return (
                <div className='card'>
                    <div className='card-header'>
                        <center>
                            <h4>
                                No Posts Match
                            </h4>
                        </center>
                    </div>
                    <div className='card-body'>
                        <center>
                            No posts matching the tag could be found.
                        </center>
                    </div>
                </div>
            );
        }
        var listing = [];
        for(var i = 0; i < json.length; i += 2) {
            if(i + 1 < json.length) {
                listing.push(
                    <div className='row' key={i/2}>
                        <Card {...json[i]} />
                        <Card {...json[i+1]} />
                    </div>
                );
            } else {
                listing.push(
                    <div className='row' key={i/2}>
                        <Card {...json[i]} />
                    </div>
                );
            }
        }
        return (
            <div className='listing'>
                {listing}
            </div>
        );
    }

};

export default Listing;
