import {useSelector, useDispatch} from 'react-redux'
import {Robot, Stars, WrenchAdjustable} from "react-bootstrap-icons";

import * as Dialog from "../components/Dialog.tsx";
import {RootState} from "../store.ts";
import {close} from '../state/settings.ts'

export function SettingsDialog() {
    const isOpen = useSelector((state: RootState) => state.settings.open);
    const dispatch = useDispatch();

    return <Dialog.Dialog open={isOpen}
                          onClose={() => dispatch(close())}
                          title='Configure settings'>
        <p className="text-sm font-normal text-gray-500 dark:text-gray-400">
            Configure models, or plugins to use in conversations with AI agents.
        </p>
        <ul className="my-4 space-y-3">
            <li>
                <a href="#"
                   className="flex items-center p-3 text-base font-bold text-gray-900 rounded-lg bg-gray-50 hover:bg-gray-100 group hover:shadow dark:bg-gray-600 dark:hover:bg-gray-500 dark:text-white">
                    <Stars className='size-5'/>
                    <span className="flex-1 ms-3 whitespace-nowrap">Models</span>
                </a>
            </li>
            <li>
                <a href="#"
                   className="flex items-center p-3 text-base font-bold text-gray-900 rounded-lg bg-gray-50 hover:bg-gray-100 group hover:shadow dark:bg-gray-600 dark:hover:bg-gray-500 dark:text-white">
                    <Robot className='size-5'/>
                    <span className="flex-1 ms-3 whitespace-nowrap">Model plugins</span>
                </a>
            </li>
            <li>
                <a href="#"
                   className="flex items-center p-3 text-base font-bold text-gray-900 rounded-lg bg-gray-50 hover:bg-gray-100 group hover:shadow dark:bg-gray-600 dark:hover:bg-gray-500 dark:text-white">
                    <WrenchAdjustable className='size-5'/>
                    <span className="flex-1 ms-3 whitespace-nowrap">Tool plugins</span>
                </a>
            </li>
        </ul>
    </Dialog.Dialog>;
}