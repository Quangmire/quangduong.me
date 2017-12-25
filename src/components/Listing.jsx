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
            data: []
        };
        this.get_posts();
        this.width = 0;
        // Default enable sidebar on larger screens
        $(document).ready(function() {
            this.width = $(window).width();
        }.bind(this));
        // Drop sidebar if screen gets too small and restore afterwards
        $(window).resize(function() {
            if($(window).width() !== this.width) {
                if(($(window).width() <= 800 && this.width > 800) ||
                    ($(window).width() > 800 && this.width <= 800)) {
                    this.forceUpdate();
                }
            }
            this.width = $(window).width();
        }.bind(this));
    }

    get_posts() {
        fetch('/posts.json', {method: 'GET'})
            .then(response => response.json())
            .then(json => {
                this.setState({
                    data: json
                });
            });
    }

    componentDidUpdate() {
        MathJax.Hub.Queue(["Typeset",MathJax.Hub]);
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

        function Date(props) {
            if(props.date) {
                return (
                    <div className='card-date'>
                        {props.date}
                    </div>
                );
            }
        }

        function Header(props) {
            if(props.github) {
                return (
                    <div className='card-header'>
                        <Github github={props.github} />
                        <h4>
                            <Link className='page-link' to={props.link}>
                                {props.title}
                            </Link>
                        </h4>
                        <Date date={props.date} />
                    </div>
                );
            }
            return (
                <div className='card-header'>
                    <h4>
                        <Link className='page-link' to={props.link}>
                            {props.title}
                        </Link>
                    </h4>
                    <Date date={props.date} />
                </div>
            );
        }

        function Card(props) {
            return (
                <div className='card'>
                    <Header {...props} />
                    <Markdown className='card-body' markdown={props.text.join('\n')}/>
                    <div className='row'>
                        <Tags tags={props.tags} />
                        <ReadMore link={props.link} />
                    </div>
                </div>
            );
        }

        var tag = this.props.type;
        if(tag === 'tag') {
            tag = window.location.pathname.split('/tag/')[1];
        }
        var json = this.state.data.filter(function(datum) {
            return datum.tags.includes(tag);
        }, this);

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

        if($(window).width() > 800) {
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
        } else {
            return (
                <div className='listing'>
                    {json.map(function(card, i) {
                        return (
                            <Card {...card} key={i} />
                        );
                    })}
                </div>
            );
        }
    }
};

export default Listing;
