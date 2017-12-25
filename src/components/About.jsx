import React from 'react'
import {
    BrowserRouter as Router,
    Route,
    Link
} from 'react-router-dom'

class About extends React.Component {

    constructor(props) {
        super(props);
    }

    render() {
        return (
            <div className='about'>
                <div className='card'>
                    <div className='card-header'>
                        <h2>Quang Duong</h2>
                    </div>
                    <div className='card-body'>
                        <img className='about-img' src='/static/media/profile.png'/>
                        <div className='about-desc'>
                            <p><strong>Bio</strong><br />
                                A third year Computer Science/Mathematics major
                                at UT Austin that enjoys reading anything from
                                high fantasy to romance.
                            </p>
                        </div>
                    </div>
                </div>
            </div>
        );
    }
}

export default About;
