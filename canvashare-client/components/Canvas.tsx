'use client';

import React from 'react'
import { FC, useEffect, useState } from 'react';
import { ChromePicker } from 'react-color';
import { v4 as uuidv4 } from 'uuid';
import { drawLine } from '@/utils/drawLine';
import { useDraw } from '@/hooks/useDraw';

const ws = new WebSocket("ws://localhost:8080/ws/");

interface pageProps {}

const Canvas: FC<pageProps> = ({}) => {

    const [color, setColor] = useState<string>('#000');
    const {canvasRef, onMouseDown, clear} = useDraw(createLine);

    function createLine({ prevPoint, currentPoint, ctx }: Draw) {
        const data = {prevPoint, currentPoint, color};
        const msg = JSON.stringify({ event: 'draw-line', data });
        ws.send(msg);
        drawLine({ prevPoint, currentPoint, ctx, color});
      }
    
      useEffect(() => {
        const ctx = canvasRef.current?.getContext('2d');
        // Indicate client is ready for bi-directional communication
        ws.onopen = () => {
          const join = {
            id: 1,
            room_name: 'main'
          };
          ws.send(JSON.stringify(join));
        }
    
        ws.addEventListener('ping', function (event) {
            console.log('received ping:', event);
        });
    
        // Example echo message. TO DELETE
        ws.onmessage = (msg: any) => {
          const messageParse = msg.data;
          try {
            const messageParseObj = JSON.parse(messageParse);
            switch(messageParseObj.event) {
              case 'draw-line':
                const drawData = messageParseObj.data;
                console.log(messageParseObj.data);
                if(ctx) {
                  drawLine({ 
                    prevPoint: drawData.prevPoint, 
                    currentPoint: drawData.currentPoint, 
                    ctx, 
                    color: drawData.color});
                }
                break;
              case 'clear-canvas':
                clear();
              default:
                break;
            }
          } catch(e) {
            console.log("Error parsing message: ", e);
          }
        }
    
        ws.onclose = () => {
          console.log("Disconnected from WebSocket server");
        }
      }, [canvasRef]);

    return (
        <div className="w-screen h-screen bg-white flex justify-center items-center">
            <div className="flex flex-col gap-10 pr-10">
                <ChromePicker color={color} onChange={(e) => setColor(e.hex)} />
                <button
                type='button'
                className="p-2 rounded-md border border-black"
                onClick={() => {
                  ws.send(JSON.stringify({ event: "clear-canvas", data: null }));
                  clear();
                }}>
                    Clear Canvas
                </button>
            </div>
            <canvas
                ref={canvasRef}
                onMouseDown={onMouseDown}
                width={1300}
                height={900}
                className="border border-black-rounded-md"
            />
        </div>
    )
}

export default Canvas