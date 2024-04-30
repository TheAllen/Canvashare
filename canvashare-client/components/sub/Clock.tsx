'use client';
import React, { useState, useEffect } from 'react';

const Clock = () => {
    const getCurrentTime = () => {
        return new Date().toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' });
    };
    const [time, setTime] = useState(getCurrentTime());


    useEffect(() => {
        const timerId = setInterval(() => setTime(getCurrentTime()), 30000);

        return () => {
            clearInterval(timerId);
        }
    }, []);

    return (
        <div className='text-sm'>
            {time}
        </div>
    )
}

export default Clock;