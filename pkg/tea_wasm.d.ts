/* tslint:disable */
/* eslint-disable */
/**
* @param {Uint8Array} str
* @param {string} key_base64
* @param {number} iter
* @returns {Uint8Array}
*/
export function tea_encrypt(str: Uint8Array, key_base64: string, iter: number): Uint8Array;
/**
* @param {Uint8Array} str
* @param {string} key_base64
* @param {number} iter
* @returns {Uint8Array}
*/
export function tea_decrypt(str: Uint8Array, key_base64: string, iter: number): Uint8Array;
/**
* @param {string} str
* @returns {string}
*/
export function base64_encrypt(str: string): string;
/**
* @param {string} str
* @returns {Uint8Array}
*/
export function base64_decrypt(str: string): Uint8Array;
