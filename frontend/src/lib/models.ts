import { z } from "zod";

export const QuestionOptionModel = z.object({
	id: z.uuidv4(),
	question_id: z.string(),
	key: z.string(),
	value: z.string()
});
export type QuestionOption = z.infer<typeof QuestionOptionModel>;

export const QuestionTypeModel = z.enum(["MCQ", "INTEGER"]);
export type QuestionType = z.infer<typeof QuestionTypeModel>;

export const QuestionModel = z.object({
	id: z.uuidv4(),
	chapter_id: z.string(),
	question_type: QuestionTypeModel,
	content: z.string(),
	options: z.array(QuestionOptionModel).default([]),
	answer: z.string()
});
export type Question = z.infer<typeof QuestionModel>;

export const ChapterModel = z.object({
	id: z.uuidv4(),
	subject_id: z.string(),
	title: z.string(),
	total_questions: z.number(),
	questions: z.array(QuestionModel).default([])
});
export type Chapter = z.infer<typeof ChapterModel>;

export const SubjectModel = z.object({
	id: z.uuidv4(),
	exam_id: z.string(),
	title: z.string(),
	total_chapters: z.number(),
	chapters: z.array(ChapterModel).default([])
});
export type Subject = z.infer<typeof SubjectModel>;

export const ExamModel = z.object({
	id: z.uuidv4(),
	title: z.string(),
	total_subjects: z.number(),
	subjects: z.array(SubjectModel).default([])
});
export type Exam = z.infer<typeof ExamModel>;

export const RaesanTestModel = z.object({
	id: z.uuidv4(),
	created_at: z.number().int().default(0),
	total_question: z.number().int().default(10),
	total_mcq_questions: z.number().int().default(1),
	total_integer_questions: z.number().int().default(1),
	questions: z.array(QuestionModel).default([])
});
export type RaesanTest = z.infer<typeof RaesanTestModel>;
