import React from 'react'
import {
    BrowserRouter as Router,
    Route,
    Link,
    Redirect
} from 'react-router-dom'
import {
    Collapse,
    NavbarToggler,
    Navbar,
    NavItem,
    NavbarBrand,
    Nav,
    NavLink,
    Col,
    Row
} from 'reactstrap';
import $ from 'jquery'

export class NavBar extends React.Component {

    render() {
        return (
            <Navbar dark className='nav-stacked bg-dark' id='navbar'>
                <Nav id='navbar-content' vertical>
                        <NavItem>
                            <NavLink id='nav-home' tag={Link} to='/'>Home</NavLink>
                        </NavItem>
                        <NavItem>
                            <NavLink id='nav-projects' tag={Link} to='/projects'>Projects</NavLink>
                        </NavItem>
                        <NavItem>
                            <NavLink id='nav-chronocides' tag={Link} to='/chronocides'>Chronocides</NavLink>
                        </NavItem>
                        <NavItem>
                            <NavLink id='nav-archives' tag={Link} to='/archives'>Archives</NavLink>
                        </NavItem>
                        <NavItem>
                            <NavLink id='nav-about' tag={Link} to='/about'>About Me</NavLink>
                        </NavItem>
                        <NavItem>
                            <NavLink id='nav-contact' tag={Link} to='/contact'>Contact</NavLink>
                        </NavItem>
                </Nav>
            </Navbar>
        );
    }
}


export class SideBar extends React.Component {

    constructor(props) {
        super(props);
        this.width = 0;
        // Default enable sidebar on larger screens
        $(document).ready(function() {
            if($(window).width() > 800 && !$('#sidebar').hasClass('active')) {
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
            '/projects': '',
            '/chronocides': '',
            '/archives': '',
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
            <Navbar dark className='nav-stacked bg-dark' id='sidebar'>
                <div id='sidebar-content-container'>
                    <nav id='sidebar-content'>
                        <NavItem>
                            <NavbarBrand id='side-logo' href='/'>Quangmire's Musings</NavbarBrand>
                        </NavItem>
                        <NavItem>
                            <NavLink id='side-home' tag={Link} className={home} to='/'>Home</NavLink>
                        </NavItem>
                        <NavItem>
                            <NavLink id='side-projects' tag={Link} className={links['/projects']} to='/projects'>Projects</NavLink>
                        </NavItem>
                        <NavItem>
                            <NavLink id='side-chronocides' tag={Link} className={links['/chronocides']} to='/chronocides'>Chronocides</NavLink>
                        </NavItem>
                        <NavItem>
                            <NavLink id='side-archives' tag={Link} className={links['/archives']} to='/archives'>Archives</NavLink>
                        </NavItem>
                        <NavItem>
                            <NavLink id='side-about' tag={Link} className={links['/about']} to='/about'>About Me</NavLink>
                        </NavItem>
                        <NavItem>
                            <NavLink id='side-contact' tag={Link} className={links['/contact']} to='/contact'>Contact</NavLink>
                        </NavItem>
                    </nav>
                </div>
            </Navbar>
        );
    }
}

export class TitleBar extends React.Component {

    constructor(props) {
        super(props);
        this.toggle = props.toggle;
    }

    render() {
        var titles = {
            '/projects': 'Projects',
            '/chronocides': 'Chronocides',
            '/archives': 'Archives',
            '/about': 'About Me',
            '/contact': 'Contact'
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
            <Navbar light id='titlebar'>
                <NavbarToggler id='sidebar-collapse' onClick={this.toggle} />
                <Col>
                    <center>
                        <h3>{title}</h3>
                    </center>
                </Col>
            </Navbar>
        );
    }

}
