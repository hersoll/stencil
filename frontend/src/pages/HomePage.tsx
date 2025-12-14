import { useTranslation } from 'react-i18next';
import CourseButton from '../components/CourseButton';
import i18n from "../i18n";

export default function HomePage() {
  const { t } = useTranslation();

  return (
    <>
      <h1 className='mb-4'>Stencil</h1>
      <h2 className='mb-4'>Välj kurs</h2>
      <div className='flex gap-4'>
        <CourseButton course={t("courses.ma1b")} page="ma1b" />
        <CourseButton course={t("courses.ma2b")} page="ma2b" />
      </div>

      <button onClick={() => { i18n.changeLanguage("sv"); localStorage.setItem("lang", "sv") }}>
        Svenska
      </button>
      <button onClick={() => { i18n.changeLanguage("en"); localStorage.setItem("lang", "en") }}>
        English
      </button>
    </>
  )
}
