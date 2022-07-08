import {open} from '@tauri-apps/api/dialog'
import {useField} from 'formik'
import React, {useCallback} from 'react'
import styled, {css} from 'styled-components'
import {Field} from '../../../Field'
import {FormFieldProps} from '../../../Field/FormField.types'
import {FormikFieldStatus} from '../../Formik.field.status'
  
export const formikSelectFileStyles = css`
  button {
    border: none;
    box-shadow: none;
    outline: none;
    padding: 0.5em 1em;
    border: solid 1px;
    color: ${props => props.theme.colors.grey.val};

    &:hover {
      color: ${props => props.theme.colors.primary.val};
    }
  }
`
  
const FormikSelectFileView = styled.div`
  ${ formikSelectFileStyles }
`
  
export type FormikSelectFileProps = {
  name: string
  config: FormFieldProps
}
export const FormikSelectFile = ({name, config}: FormikSelectFileProps): JSX.Element => {
  const [{value}, {touched}, {setValue, setTouched}] = useField({name})

  const handler = useCallback(async (e: React.MouseEvent<HTMLButtonElement, MouseEvent>) => {
    e.preventDefault()
    const path = await open({
      multiple: false,
      title: 'Select SSH Key'
    })

    if(!touched) setTouched(true)

    if(path) setValue(path as string)

  },[touched])

  return (
    <FormikSelectFileView>
      <Field suppressStyles {...config}>
        <button onClick={handler}>{value || 'Select File...'}</button>
      </Field>
      <FormikFieldStatus name={name} />
    </FormikSelectFileView>
  )
}