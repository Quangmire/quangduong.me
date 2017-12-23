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
                    <div className='card-body'>
                        <img className='about-img' src='/static/media/profile.png'/>
                        <div className='about-desc'>
                            <h2>Quang Duong</h2>
                            <p><strong>Bio:</strong> Quang is a third year CS/Math student with an addiction to reading anything from fantasy to romance.</p>
                        </div>
                    </div>
                </div>
            </div>
        );
    }
}

export default About;
