//! This module is responsible for reading the logging DBs and calculate different stats from that
//! data.

// # What do we want to do?
//
// - How many api calls/percentages per language? Over 24 hours, 7 days, 30 days, 365 days, all time
// - Which courses (absolute, percentages)?
// - PDF count timeline, per hour over 24 hours, week, per day over month, etc.
// - For EVERY column in stats_pdf and stats_sets, show a pie chart (with time selection)
// - (Rename DB tables to logs_*)
// - Pie chart over topics per set, set count per pdf
//
// ## Leaderboards:
// - Most used chapters
// - Most used topics
// - Most excluded problems
// - Best and worst render times
// - Busiest days
//
// ## Averages (with timeline):
// - Render time
// - Topics per set
// - Problem count per set
// - Set count per pdf
//
// NOTE: We will probably need some sort of Duration enum (std is already called Duration...)
//
// NOTE: We also want to either A: make the site private or B: not generate stats on request, but rather
// generate it with set intervals and simply show the stored data
// Leaning towards A....
//
// Time the stat generation!
