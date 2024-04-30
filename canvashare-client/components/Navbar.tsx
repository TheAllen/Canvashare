'use client';

import React from 'react';
import Clock from './sub/Clock';

const Navbar = () => {
    return (
        <div className='bg-gray-800 text-white flex justify-between items-center p-4'>
            <h1 className='text-lg font-semibold'>Canvashare</h1>
            <Clock />
        </div>
    )
}

export default Navbar;