import React from 'react'
import {
    BrowserRouter as Router,
    Route,
    Link,
    Redirect,
    Switch
} from 'react-router-dom'
import $ from 'jquery';
import {
    TitleBar,
    SideBar,
    NavBar
} from './components/NavBar.jsx';
import About         from './components/About.jsx';
import Page          from './components/Page.jsx';
import NotFound      from './components/NotFound.jsx';
import WIP           from './components/WIP.jsx';
import Listing       from './components/Listing.jsx';

class App extends React.Component {

    constructor(props) {
        super(props);
        this.state = {
            is_open: true
        };
    }

    render() {
        return (
            <Router>
                <div id='wrapper'>
                    <SideBar/>
                    <div id='content'>
                        <NavBar />
                        <TitleBar toggle={this.toggle} />
                        <Switch>
                            <Route exact path='/' component={(props) => <Page type='frontpage' />} />
                            {/* Listings */}
                            <Route exact path='/posts' component={(props) => <Listing type='blog' />}/>
                            <Route exact path='/projects' component={(props) => <Listing type='project' />}/>
                            <Route exact path='/chronocides' component={(props) => <Listing type='chronocide' />}/>
                            <Route exact path='/notes' component={(props) => <Listing type='note' />}/>
                            <Route path='/tag/:tag' component={(props) => <Listing type='tag' />}/>
                            {/* Pages */}
                            <Route path='/posts/:name' component={Page}/>
                            <Route path='/projects/:name' component={Page}/>
                            <Route path='/chronocides/:name' component={Page}/>
                            <Route path='/notes/:name' component={Page}/>
                            {/* Other Pages */}
                            <Route exact path='/about' component={About}/>
                            {/* These pages are not yet implemented */}
                            <Route exact path='/contact' component={WIP}/>
                            {/* 404 Page */}
                            <Route exact path='/404' component={NotFound} />
                            <Redirect to="/404" />
                        </Switch>
                    </div>
                </div>
            </Router>
        );
    }
}

export default App;
