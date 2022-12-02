import React, { useEffect, useRef, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";

function App() {

  const [favorites, setFavorites] = useState([] as string[])
  const boxNumber = useRef<HTMLInputElement>(null)

  async function swap(box: number) {
    try {
      await invoke("swap", {boxNumber: box});
      console.log(`swapped to ${box}`);
    } catch (error) {
      console.error(error);
      return
    }
  }

  async function favorite(selector: string) {
    try {
      await invoke("favorite", {selector: selector});
      console.log(`swapped to favorite ${selector}`);
    } catch (error) {
      console.error(error);
      return
    }
  }

  async function getFavorites() {
    let favorites: string[]
    try {
      favorites = await invoke("get_favorites");
    } catch (error: any) {
      console.error(error);
      throw error
    }
    return favorites
  }

  useEffect(() => {
    getFavorites().then(f => setFavorites(f)).catch(error =>console.error(error))
  }, [])
  
  return (
    <div className="container">
      <div className="row">
        <input ref={boxNumber} type="number"></input>
        <button onClick={() => {
          let boxNum = boxNumber.current!.value
          swap(Number.parseInt(boxNum))
        }}>Swap Devbox</button>
      </div>
      <div className="groupbox">
        {favorites.map(fav => <button key={fav} onClick={() => favorite(fav)}>{fav}</button>)}
      </div>
    </div>
  );
}

export default App;
