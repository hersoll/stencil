import { useTranslation } from 'react-i18next';
import CourseButton from '../components/CourseButton';
import i18n from "../i18n";

export default function HomePage() {
  const { t } = useTranslation();

  return (
    <>
      <h1 className='mb-4'>Stencil</h1>
      <div className='bg-mid p-4 mb-10 rounded-xl'>
        <h2 className='mb-2 text-2xl text-left'>{t("home_header")}</h2>
        <p className='text-muted text-left'>{t("home_par")}</p>
        <div className='flex gap-4 mt-4'>
          <CourseButton course={t("courses.ma1b")} page="1b" />
          <CourseButton course={t("courses.ma2b")} page="2b" />
        </div>
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
