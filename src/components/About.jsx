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
                        <div className='about-desc'>
                            <img className='about-img' src='/static/media/profile.png'/>
                            <p><strong>Bio</strong><br />
                                A third year Computer Science and Mathematics 
                                (with a specialization in Scientific Computing)
                                student at UT Austin that enjoys reading 
                                anything from high fantasy to romance, belting 
                                out songs in the shower whenever he is fully 
                                convinced there is no one around, and indulging 
                                in and subjecting others to the world&#39;s 
                                highest form of humor--puns. He can commonly be
                                found ricing his Arch distro in procrastination
                                of upcoming deadlines.
                            </p>
                        </div>
                        <div className='about-desc'>
                            <strong>Interests</strong>
                            <ul>
                                <li>
                                    Research in Machine Learning/Data Science
                                </li>
                                <li>
                                    Procedural Generation/Graphics
                                </li>
                                <li>
                                    Security and Low-Level Hackery
                                </li>
                            </ul>
                            <p><strong>Resume</strong><br />
                                To be added at a later date.
                            </p>
                        </div>
                    </div>
                </div>
            </div>
        );
    }
}

export default About;
