import React from 'react'
import {
    BrowserRouter as Router,
    Route,
    Link,
    Redirect } from 'react-router-dom'
import $ from 'jquery'

export class NavBar extends React.Component {

    render() {
        return (
            <nav id='navbar'>
                <nav id='navbar-content'>
                    <Link className='nav-link' to='/'>
                        Home
                    </Link>
                    <Link className='nav-link' to='/posts'>
                        Archives
                    </Link>
                    <Link className='nav-link' to='/projects'>
                        Projects
                    </Link>
                    <Link className='nav-link' to='/chronocides'>
                        Chronocides
                    </Link>
                    <Link className='nav-link' to='/notes'>
                        Notes
                    </Link>
                    <Link className='nav-link' to='/about'>
                        About Me
                    </Link>
                    <Link className='nav-link' to='/contact'>
                        Contact
                    </Link>
                </nav>
            </nav>
        );
    }
}


export class SideBar extends React.Component {

    constructor(props) {
        super(props);
        this.width = 0;
        // Default enable sidebar on larger screens
        $(document).ready(function() {
            if($(window).width() > 800) {
                $('#sidebar').addClass('active');
            }
            this.width = $(window).width();
        }.bind(this));
        // Drop sidebar if screen gets too small and restore afterwards
        $(window).resize(function() {
            if($(window).width() !== this.width) {
                if($(window).width() <= 800) {
                    if($('#sidebar').hasClass('active')) {
                        $('#sidebar').removeClass('active');
                    }
                } else if($(window).width() > 800) {
                    if(!$('#sidebar').hasClass('active')) {
                        $('#sidebar').addClass('active');
                    }
                    if($('#navbar').hasClass('active')) {
                        $('#navbar').removeClass('active');
                    }
                }
            }
            this.width = $(window).width();
        }.bind(this));
    }

    render() {
        var links = {
            '/posts': '',
            '/projects': '',
            '/chronocides': '',
            '/notes': '',
            '/about': '',
            '/contact': ''
        };
        var home = '';
        var path = window.location.pathname;
        if(path === '/') {
            home = 'sidebar-active-tab';
        } else {
            for(var key in links) {
                if(path.startsWith(key)) {
                    if(path.length > key.length && path[key.length] !== '/') {
                        continue;
                    }
                    links[key] = 'sidebar-active-tab';
                }
            }
        }
        return (
            <div id='sidebar'>
                <div id='sidebar-flex'/>
                <nav id='sidebar-container'>
                    <nav id='sidebar-content'>
                        <Link className={'nav-logo nav-link ' + home} to='/'>
                            Quangmire&#39;s Musings
                        </Link>
                        <Link className={'nav-link ' + links['/posts']} to='/posts'>
                            Archives
                        </Link>
                        <Link className={'nav-link ' + links['/projects']} to='/projects'>
                            Projects
                        </Link>
                        <Link className={'nav-link ' + links['/chronocides']} to='/chronocides'>
                            Chronocides
                        </Link>
                        <Link className={'nav-link ' + links['/notes']} to='/notes'>
                            Notes
                        </Link>
                        <Link className={'nav-link ' + links['/about']} to='/about'>
                            About Me
                        </Link>
                        <Link className={'nav-link ' + links['/contact']} to='/contact'>
                            Contact
                        </Link>
                    </nav>
                </nav>
            </div>
        );
    }
}

export class TitleBar extends React.Component {

    constructor(props) {
        super(props);
    }

    toggle() {
        if($(window).width() > 800) {
            if($('#navbar').hasClass('active')) {
                $('#navbar').removeClass('active');
            }
            $('#sidebar').toggleClass('active');
        } else {
            if($('#sidebar').hasClass('active')) {
                $('#sidebar').removeClass('active');
            }
            $('#navbar').toggleClass('active');
        }
    }

    render() {
        var tag = window.location.pathname.split('/tag/')
        if(tag.length > 1) {
            tag = tag[1];
        }
        var titles = {
            '/posts': 'Archives',
            '/projects': 'Projects',
            '/chronocides': 'Chronocides',
            '/notes': 'Notes',
            '/about': 'About Me',
            '/contact': 'Contact',
            '/tag': 'Tagged: ' + tag
        };
        var title = 'Error';
        var path = window.location.pathname;
        if(path === '/') {
            title = 'Home'
        } else {
            for(var key in titles) {
                if(path.startsWith(key)) {
                    if(path.length > key.length && path[key.length] !== '/') {
                        continue;
                    }
                    title = titles[key];
                    break;
                }
            }
        }
        return (
            <div id='titlebar'>
                <div id='titlebar-content'>
                    <a id='nav-collapse' onClick={this.toggle}>
                        <i className='fa fa-bars' aria-hidden='true'></i>
                    </a>
                    <div id='title'><h2>{title}</h2></div>
                </div>
            </div>
        );
    }

}
