import { writable, derived } from 'svelte/store';

/** Store for your data. 
This assumes the data you're pulling back will be an array.
If it's going to be an object, default this to an empty object.
**/
export const apiData = writable([]);

/** Data transformation.
**/
export const players = derived(apiData, ($apiData) => {
    if ($apiData.players){
        return $apiData.players;
    }
    return [];
});

export const world = derived(apiData, ($apiData) => {
    if ($apiData.world){
        return $apiData.world;
    }
    return [];
});

export const server = derived(apiData, ($apiData) => {
    if ($apiData.server){
        return $apiData.server;
    }
    return [];
});